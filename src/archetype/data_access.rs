use super::Archetype;
use crate::archetype::metadata::EntityMetadata;
use crate::component_group::*;
use crate::{
    DEFAULT_ARCHETYPE_ALLOCATION_SIZE, MAX_COMPONENTS_PER_ENTITY, MAX_ENTITIES_PER_ARCHETYPE,
};
use alloc::alloc::{alloc, dealloc, realloc, Layout};
use core::mem::{align_of, size_of};
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

impl Archetype {
    /// Returns a tuple of component slices to the archetype's data.
    /// # Safety:
    /// - Must be called exactly with the component group contained in the archetype.
    /// - a compatible group type is also accepted.
    /// - [`G`] must have a valid archetype descriptor.
    #[inline(always)]
    pub(crate) unsafe fn get_slices_unchecked_exact<'a, G: ComponentGroup<'a>>(
        &'a self,
    ) -> G::SliceRefTuple {
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );

        G::slice_unchecked(&self.pointers, self.entity_count as usize)
    }

    /// Returns a tuple of mutable component slices to the archetype's data.
    /// # Safety:
    /// - Must be called exactly with the component group contained in the archetype.
    /// - a compatible group type is also accepted.
    /// - [`G`] must have a valid archetype descriptor.
    #[inline(always)]
    pub(crate) unsafe fn get_slices_unchecked_exact_mut<'a, G: ComponentGroup<'a>>(
        &'a mut self,
    ) -> G::SliceMutRefTuple {
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );

        G::slice_unchecked_mut(&self.pointers, self.entity_count as usize)
    }

    /// Returns the slices for the components in [`G`], provided that archetype itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the archetype.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_fuzzy_slices_unchecked<'s, G: ComponentGroup<'s>>(
        &'s self,
    ) -> G::SliceRefTuple {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>();
        G::slice_unchecked(&pointers, self.entity_count as usize)
    }

    /// Returns the mutable slices for the components in [`G`], provided that archetype itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the archetype.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_fuzzy_slices_unchecked_mut<'s, G: ComponentGroup<'s>>(
        &'s mut self,
    ) -> G::SliceMutRefTuple {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>();
        G::slice_unchecked_mut(&pointers, self.entity_count as usize)
    }
}

impl Archetype {
    /// Returns the amount of entities currently stored in the archetype.
    pub fn size(&self) -> u32 {
        self.entity_count
    }

    /// Returns the current capacity of the archetype.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns whether the archetype is full or not.
    pub fn is_full(&self) -> bool {
        self.entity_count == self.capacity
    }

    /// Returns a reference to the internal slice storing entity metadata.
    pub(crate) fn entity_metadata(&self) -> &[EntityMetadata] {
        unsafe { &*slice_from_raw_parts(self.entity_metadata, self.entity_count as usize) }
    }

    /// Returns a mutable reference to the internal slice storing entity metadata.
    pub(crate) fn entity_metadata_mut(&mut self) -> &mut [EntityMetadata] {
        unsafe { &mut *slice_from_raw_parts_mut(self.entity_metadata, self.entity_count as usize) }
    }

    /// Pushes a given entity/component-tuple into the archetype's backing memory.
    /// # Safety:
    /// - Must be called exactly with the component group contained in the archetype.
    /// - a compatible group type is also accepted.
    /// - Does not call drop on the given entity.
    /// - Increases the size of the archetype's memory allocations if required.
    /// - If resizing fails, this function will panic.
    pub(crate) unsafe fn push_entity_unchecked<'a, G: ComponentGroup<'a>>(
        &mut self,
        metadata: EntityMetadata,
        entity: G,
    ) -> u32 {
        debug_assert!(G::DESCRIPTOR.is_valid());
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );

        if self.is_full() {
            let additional_capacity = if self.capacity == 0 {
                DEFAULT_ARCHETYPE_ALLOCATION_SIZE
            } else {
                self.capacity as usize
            };
            self.resize_capacity(additional_capacity as isize);
        }

        let entity_index = self.entity_count;
        self.write_entity_unchecked(entity_index, metadata, entity);
        self.entity_count += 1;
        entity_index
    }

    /// Writes a given entity/component-tuple into the archetype's backing memory.
    /// # Safety:
    /// - Must be called exactly with the component group contained in the archetype.
    /// - a compatible group type is also accepted.
    /// - Does not call drop on the given entity.
    /// - Does not call drop on the entity that already exists at [`index`].
    /// - Assumes the underlying backing memory is sized accordingly to fit the data.
    /// - Does not increase the entity counter.
    /// - Does not check if [`index`] is out of bounds or not.
    pub(crate) unsafe fn write_entity_unchecked<'a, G: ComponentGroup<'a>>(
        &mut self,
        index: u32,
        metadata: EntityMetadata,
        mut entity: G,
    ) {
        debug_assert!(G::DESCRIPTOR.is_valid());
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );
        let mut pointers = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        entity.as_sorted_pointers(&mut pointers);
        for i in 0..G::DESCRIPTOR.len() as usize {
            let component = G::DESCRIPTOR.archetype().components().get_unchecked(i);
            let dst_pointer = self
                .pointers
                .get_unchecked(i)
                .offset(component.size as isize * index as isize);

            core::ptr::copy_nonoverlapping::<u8>(
                *pointers.get_unchecked(i),
                dst_pointer,
                component.size as usize,
            );
        }
        *self.entity_metadata_mut().get_unchecked_mut(index as usize) = metadata;
        core::mem::forget(entity);
    }
}

impl Archetype {
    /// Resizes the backing memory by some amount. If this becomes less than or equal to zero,
    /// deallocates all memory.
    /// # Safety:
    /// - Does not call drop on the entities in the backing storage.
    /// - Panics if resizing fails for whatever reason. This leaves the archetype in an undefined state.
    /// - Deallocates if the new capacity is smaller or equal to zero.
    /// - Deallocates if the new capacity exceeds [`MAX_ENTITIES_PER_ARCHETYPE`]. TODO: This is weird?
    pub(super) unsafe fn resize_capacity(&mut self, change_in_entity_count: isize) {
        let old_capacity = self.capacity;
        let new_capacity = (old_capacity as isize + change_in_entity_count);
        if new_capacity <= 0 || new_capacity >= MAX_ENTITIES_PER_ARCHETYPE as isize {
            self.dealloc();
            return;
        }
        let new_capacity = new_capacity as usize;

        let layout = Layout::from_size_align_unchecked(
            size_of::<EntityMetadata>() * old_capacity as usize,
            align_of::<EntityMetadata>(),
        );
        self.entity_metadata = realloc(
            self.entity_metadata as *mut u8,
            layout,
            size_of::<EntityMetadata>() * new_capacity,
        ) as *mut EntityMetadata;
        assert_ne!(self.entity_metadata, core::ptr::null_mut());
        for (index, pointer) in self.pointers[0..self.descriptor.len() as usize]
            .iter_mut()
            .enumerate()
        {
            let component_type = &self.descriptor.components()[index];
            let layout = alloc::alloc::Layout::from_size_align_unchecked(
                component_type.size as usize * old_capacity as usize,
                component_type.align as usize,
            );
            *pointer = realloc(
                *pointer,
                layout,
                component_type.size as usize * new_capacity,
            );
            assert_ne!(*pointer, core::ptr::null_mut());
        }
        self.capacity = new_capacity as u32;
    }

    /// Deallocates the backing memory and sets capacity to zero.
    /// # Safety:
    /// - Does not call drop on the entities in the backing storage.
    pub(super) unsafe fn dealloc(&mut self) {
        for (index, pointer) in self.pointers[0..self.descriptor.len() as usize]
            .iter_mut()
            .enumerate()
        {
            if *pointer == core::ptr::null_mut() {
                return;
            }
            let component_type = &self.descriptor.components()[index];
            let layout = alloc::alloc::Layout::from_size_align_unchecked(
                component_type.size as usize * self.capacity as usize,
                component_type.align as usize,
            );
            dealloc(*pointer, layout);
            *pointer = core::ptr::null_mut();
        }
        let layout = Layout::from_size_align_unchecked(
            size_of::<EntityMetadata>() * self.capacity as usize,
            align_of::<EntityMetadata>(),
        );
        dealloc(self.entity_metadata as *mut u8, layout);
        self.entity_metadata = core::ptr::null_mut();
        self.capacity = 0;
    }

    /// Returns the pointers for the components in [`G`], provided that archetype itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the shard.
    unsafe fn get_fuzzy_pointers_unchecked<'a, G: ComponentGroup<'a>>(
        &'a self,
    ) -> [*mut u8; MAX_COMPONENTS_PER_ENTITY] {
        let mut pointers = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        for (index, descriptor) in G::DESCRIPTOR.archetype().components().iter().enumerate() {
            'inner_loop: for check_index in index..self.descriptor.len() as usize {
                if self
                    .descriptor
                    .components()
                    .get_unchecked(check_index)
                    .component_type_id
                    .into_u16()
                    == descriptor.component_type_id.into_u16()
                {
                    *pointers.get_unchecked_mut(index) = *self.pointers.get_unchecked(check_index);
                    break 'inner_loop;
                }
            }
        }
        pointers
    }
}
