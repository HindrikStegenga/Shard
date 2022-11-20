use super::Archetype;
use crate::*;

use crate::constants::DEFAULT_ARCHETYPE_ALLOCATION_SIZE;
use alloc::alloc::{dealloc, realloc, Layout};
use core::mem::{align_of, size_of};
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

impl Archetype {
    /// Returns a reference to a specific component.
    /// # Safety:
    /// - Component type [`C`] must be present in the archetype
    /// - panics otherwise.
    pub unsafe fn get_component_unchecked<C: Component>(&self, index: u32) -> &C {
        match self
            .descriptor
            .components()
            .binary_search_by_key(&C::ID, |e| e.component_type_id)
        {
            Ok(idx) => &*(self.pointers[idx] as *mut C).offset(index as isize),
            Err(_) => unreachable!(),
        }
    }

    /// Returns a mutable reference to a specific component.
    /// # Safety:
    /// - Component type [`C`] must be present in the archetype
    /// - panics otherwise.
    pub unsafe fn get_component_unchecked_mut<C: Component>(&mut self, index: u32) -> &mut C {
        match self
            .descriptor
            .components()
            .binary_search_by_key(&C::ID, |e| e.component_type_id)
        {
            Ok(idx) => &mut *(self.pointers[idx] as *mut C).offset(index as isize),
            Err(_) => unreachable!(),
        }
    }

    /// Returns a reference to a specific component.
    /// # Safety:
    /// - Component group type [`G`] must be a subset of the types in the archetype
    /// - panics otherwise.
    pub unsafe fn get_fuzzy_components_unchecked<'a, G: ComponentGroup>(
        &'a self,
        index: u32,
    ) -> G::RefTuple<'a> {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(index);
        G::pointers_as_ref_tuple(&pointers)
    }

    /// Returns a reference to a specific component.
    /// # Safety:
    /// - Component group type [`G`] must be a subset of the types in the archetype
    /// - panics otherwise.
    pub unsafe fn get_fuzzy_components_unchecked_mut<'a, G: ComponentGroup>(
        &'a mut self,
        index: u32,
    ) -> G::MutRefTuple<'a> {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(index);
        G::pointers_as_mut_ref_tuple(&pointers)
    }

    /// Reads a specific component from the archetype at the given index.
    /// # Safety:
    /// - Component type [`C`] must be present in the archetype
    /// - panics otherwise.
    pub unsafe fn read_component_unchecked<C: Component>(&mut self, index: u32) -> C {
        match self
            .descriptor
            .components()
            .binary_search_by_key(&C::ID, |e| e.component_type_id)
        {
            Ok(idx) => {
                core::ptr::read::<C>((self.pointers[idx] as *const C).offset(index as isize))
            }
            Err(_) => unreachable!(),
        }
    }

    /// Returns a tuple of mutable component slices to the archetype's data.
    /// # Safety:
    /// - Must be called exactly with the component group contained in the archetype.
    /// - a compatible group type is also accepted.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_slices_unchecked_exact_mut<'a, G: ComponentGroup>(
        &'a mut self,
    ) -> G::SliceMutRefTuple<'a> {
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );

        G::slice_unchecked_mut(&self.pointers, self.len() as usize)
    }

    /// Returns a tuple of component slices to the archetype's data.
    /// # Safety:
    /// - Must be called exactly with the component group contained in the archetype.
    /// - a compatible group type is also accepted.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_slices_unchecked_exact<'a, G: ComponentGroup>(
        &'a self,
    ) -> G::SliceRefTuple<'a> {
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );

        G::slice_unchecked(&self.pointers, self.len() as usize)
    }

    /// Returns the slices for the components in [`G`], provided that archetype itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the archetype.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_fuzzy_slices_unchecked<'s, G: ComponentGroup>(
        &'s self,
    ) -> G::SliceRefTuple<'s> {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(0);
        G::slice_unchecked(&pointers, self.len() as usize)
    }

    /// Returns the mutable slices for the components in [`G`], provided that archetype itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the archetype.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_fuzzy_slices_unchecked_mut<'s, G: ComponentGroup>(
        &'s mut self,
    ) -> G::SliceMutRefTuple<'s> {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(0);
        G::slice_unchecked_mut(&pointers, self.len() as usize)
    }

    /// Returns the entity slice and the  slices for the components in [`G`], provided that archetype
    /// itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the archetype.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_entity_fuzzy_slices_unchecked<'s, G: ComponentGroup>(
        &'s self,
    ) -> (&'s [Entity], G::SliceRefTuple<'s>) {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(0);
        (
            self.entities(),
            G::slice_unchecked(&pointers, self.len() as usize),
        )
    }

    /// Returns the entity slice and the  mutable slices for the components in [`G`], provided that
    /// archetype itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the archetype.
    /// - [`G`] must have a valid archetype descriptor.
    pub unsafe fn get_entity_fuzzy_slices_unchecked_mut<'s, G: ComponentGroup>(
        &'s mut self,
    ) -> (&'s [Entity], G::SliceMutRefTuple<'s>) {
        debug_assert!(G::DESCRIPTOR.is_valid());
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(0);
        (
            self.entities(),
            G::slice_unchecked_mut(&pointers, self.len() as usize),
        )
    }
}

impl Archetype {
    /// Returns the amount of entities currently stored in the archetype.
    pub fn len(&self) -> u32 {
        self.entity_count
    }

    /// Returns the current capacity of the archetype.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns whether the archetype is full or not.
    pub fn is_full(&self) -> bool {
        self.entity_count == self.capacity()
    }

    /// Returns a reference to the internal slice storing entity associations.
    pub fn entities(&self) -> &[Entity] {
        unsafe { &*slice_from_raw_parts(self.entity_associations, self.len() as usize) }
    }

    /// Returns a mutable reference to the internal slice storing entity associations.
    pub fn entities_mut(&mut self) -> &mut [Entity] {
        unsafe { &mut *slice_from_raw_parts_mut(self.entity_associations, self.len() as usize) }
    }

    /// Pushes a given entity/component-tuple into the archetype's backing memory.
    /// # Safety:
    /// - Must be called exactly with the component group contained in the archetype.
    /// - a compatible group type is also accepted.
    /// - Does not call drop on the given entity.
    /// - Increases the size of the archetype's memory allocations if required.
    /// - If resizing fails, this function will panic.
    pub unsafe fn push_entity_unchecked<'a, G: ComponentGroup>(
        &mut self,
        entity_handle: Entity,
        entity: G,
    ) -> u32 {
        debug_assert!(G::DESCRIPTOR.is_valid());
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );
        self.resize_if_necessary();
        let entity_index = self.len();
        self.entity_count += 1;
        self.write_entity_unchecked(entity_index, entity_handle, entity);
        entity_index
    }

    /// Identical to push_entity_unchecked but does not actually write the entity's component data.
    /// The memory at the the returned index MUST be written with valid component data.
    /// The metadata is not set either.
    pub unsafe fn push_uninitialized_entity(&mut self) -> u32 {
        self.resize_if_necessary();
        let entity_index = self.len();
        self.entity_count += 1;
        entity_index
    }

    /// Decrements archetype size by 1, therefore assuming the last entity is moved elsewhere.
    /// As such, it does not call drop on the last entity.
    pub unsafe fn decrement_len_unchecked(&mut self) {
        self.entity_count -= 1;
    }

    /// Writes a single component into a specific position.
    /// Does not call drop on the existing component at index.
    /// Panics if called on an archetype that does not contain [`C`].
    pub unsafe fn write_single_component_unchecked<C: Component>(
        &mut self,
        index: u32,
        component: C,
    ) {
        match self
            .descriptor
            .components()
            .binary_search_by_key(&C::ID, |e| e.component_type_id)
        {
            Ok(idx) => {
                let pointer = (self.pointers[idx] as *mut C).offset(index as isize);
                core::ptr::write(pointer, component);
            }
            Err(_) => unreachable!(),
        }
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
    pub unsafe fn write_entity_unchecked<G: ComponentGroup>(
        &mut self,
        index: u32,
        entity_handle: Entity,
        mut entity: G,
    ) {
        debug_assert!(index < self.capacity());
        debug_assert!(G::DESCRIPTOR.is_valid());
        debug_assert_eq!(
            G::DESCRIPTOR.archetype().archetype_id(),
            self.descriptor.archetype_id()
        );
        let mut pointers = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        entity.as_sorted_pointers(&mut pointers);
        for i in 0..G::DESCRIPTOR.len() as usize {
            let component = G::DESCRIPTOR
                .archetype()
                .components_unchecked()
                .get_unchecked(i);
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
        self.entities_mut()[index as usize] = entity_handle;
        core::mem::forget(entity);
    }

    /// Swaps the entity at [`index`] and the last entity and drops the now-last entity.
    /// This effectively reduces the size of the archetype by 1, dropping the entity at index.
    /// And moving the previously last entity to the position at index.
    /// If [`index`] is the last element, simply drops it instead without any swaps occurring.
    /// Returns true if a swap occurred, or false if not.
    /// # Safety:
    /// - [`index`] must be smaller than the amount of entities in the archetype.
    pub unsafe fn swap_drop_unchecked(&mut self, index: u32) -> bool {
        debug_assert!(index < self.len());
        if index == self.len() - 1 {
            // Is the last one, so just drop it.
            self.drop_entity(index);
            self.entity_count -= 1;
            false
        } else {
            self.swap_entities(index, self.len() - 1);
            self.drop_entity(self.len() - 1);
            self.entity_count -= 1;
            true
        }
    }

    /// Swaps the entity at [`index`] and the last entity.
    /// Makes sure the entity at [`index`] is at the end of the archetype.
    /// If [`index`] is the last element, does nothing.
    /// Returns true if a swap occurred, or false if not.
    /// # Safety:
    /// - [`index`] must be smaller than the amount of entities in the archetype.
    pub unsafe fn swap_to_last_unchecked(&mut self, index: u32) -> bool {
        debug_assert!(index < self.len());
        if index == self.len() - 1 {
            false
        } else {
            self.swap_entities(index, self.len() - 1);
            true
        }
    }

    /// Swaps the entity at [`index`] and the last entity and returns the now-last entity.
    /// This effectively reduces the size of the archetype by 1, returning the entity at index.
    /// And moving the previously last entity to the position at index.
    /// If [`index`] is the last element, simply returns it instead without any swaps occurring.
    /// Returns true if a swap occurred, or false if not.
    /// # Safety:
    /// - [`index`] must be smaller than the amount of entities in the archetype.
    /// - [`G`] must exactly match the type store in the archetype.
    /// - Ordering of component in [`G`] may be different.
    pub unsafe fn swap_remove_unchecked<G: ComponentGroup>(&mut self, index: u32) -> (G, bool) {
        debug_assert!(index < self.len());
        if index == self.len() - 1 {
            // Is the last one, so just drop it.
            let data: G = self.read_components_exact_unchecked::<G>(index);
            self.entity_count -= 1;
            (data, false)
        } else {
            self.swap_entities(index, self.len() - 1);
            let data: G = self.read_components_exact_unchecked(self.len() - 1);
            self.entity_count -= 1;
            (data, true)
        }
    }

    /// Swaps the entities at the provided positions.
    /// # Safety:
    /// - [`first`] must be smaller than the amount of entities in the archetype.
    /// - [`second`] must be smaller than the amount of entities in the archetype.
    /// - [`first`] must not be equal to [`last`].
    pub unsafe fn swap_entities(&mut self, first: u32, second: u32) {
        for (idx, descriptor) in self.descriptor.components().iter().enumerate() {
            let ptr_first = self.pointers[idx].offset(first as isize * descriptor.size as isize);
            let ptr_second = self.pointers[idx].offset(second as isize * descriptor.size as isize);
            core::ptr::swap_nonoverlapping(ptr_first, ptr_second, descriptor.size as usize);
        }
        self.entities_mut().swap(first as usize, second as usize);
    }

    /// Calls drop on the entity at [`index`].
    /// # Safety:
    /// - [`index`] must be smaller than the amount of entities in the archetype.
    pub unsafe fn drop_entity(&mut self, index: u32) {
        for (idx, descriptor) in self.descriptor.components().iter().enumerate() {
            (descriptor.fns.drop_handler)(
                self.pointers[idx].offset(index as isize * descriptor.size as isize),
                1,
            );
        }
    }

    /// Drops all the entities in the archetype.
    /// Does not deallocate the memory.
    pub unsafe fn drop_entities(&mut self) {
        for (idx, descriptor) in self.descriptor.components().iter().enumerate() {
            (descriptor.fns.drop_handler)(self.pointers[idx], self.len() as usize);
        }
    }

    /// Reads the component data at [`index`] and returns it.
    /// # Safety:
    /// - [`G`] must be exactly the type stored in the archetype.
    /// - a compatible one also works. (i.e. same archetype, different ordering)
    pub unsafe fn read_components_exact_unchecked<G: ComponentGroup>(&self, index: u32) -> G {
        let pointers = self.offset_sorted_pointers_unchecked(index);
        G::read_from_sorted_pointers(&pointers)
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
        let old_capacity = self.capacity();
        let new_capacity = old_capacity as isize + change_in_entity_count;
        if new_capacity <= 0 || new_capacity >= MAX_ENTITIES_PER_ARCHETYPE as isize {
            self.dealloc();
            return;
        }
        let new_capacity = new_capacity as usize;

        let layout = Layout::from_size_align_unchecked(
            size_of::<Entity>() * old_capacity as usize,
            align_of::<Entity>(),
        );
        self.entity_associations = realloc(
            self.entity_associations as *mut u8,
            layout,
            size_of::<Entity>() * new_capacity,
        ) as *mut Entity;
        assert_ne!(self.entity_associations, core::ptr::null_mut());
        for (index, pointer) in self.pointers[0..self.descriptor.len() as usize]
            .iter_mut()
            .enumerate()
        {
            let component_type = &self.descriptor.components()[index];
            let layout = Layout::from_size_align_unchecked(
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
            if pointer.is_null() {
                return;
            }
            let component_type = &self.descriptor.components()[index];
            let layout = Layout::from_size_align_unchecked(
                component_type.size as usize * self.capacity as usize,
                component_type.align as usize,
            );
            dealloc(*pointer, layout);
            *pointer = core::ptr::null_mut();
        }
        let layout = Layout::from_size_align_unchecked(
            size_of::<Entity>() * self.capacity() as usize,
            align_of::<Entity>(),
        );
        dealloc(self.entity_associations as *mut u8, layout);
        self.entity_associations = core::ptr::null_mut();
        self.capacity = 0;
    }

    /// Returns the pointers, offset by [`index`] elements.
    unsafe fn offset_sorted_pointers_unchecked(
        &self,
        index: u32,
    ) -> [*mut u8; MAX_COMPONENTS_PER_ENTITY] {
        let mut pointers = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        for (c_idx, pointer) in self.pointers[0..self.descriptor.len() as usize]
            .iter()
            .enumerate()
        {
            pointers[c_idx] =
                pointer.offset(self.descriptor.components()[c_idx].size as isize * index as isize);
        }
        pointers
    }

    /// Resizes the backing memory by the default amount if necessary.
    unsafe fn resize_if_necessary(&mut self) {
        if self.is_full() {
            let additional_capacity = if self.capacity() == 0 {
                DEFAULT_ARCHETYPE_ALLOCATION_SIZE
            } else {
                self.capacity() as usize
            };
            self.resize_capacity(additional_capacity as isize);
        }
    }

    /// Copies common components between two archetypes.
    pub unsafe fn copy_common_components_between_archetypes_unchecked(
        source: &Archetype,
        source_index: u32,
        destination: &mut Archetype,
        destination_index: u32,
    ) {
        for (source_c_idx, source_component) in source.descriptor.components().iter().enumerate() {
            for (destination_c_idx, destination_component) in
                destination.descriptor.components().iter().enumerate()
            {
                if source_component.component_type_id != destination_component.component_type_id {
                    continue;
                }
                core::ptr::copy_nonoverlapping(
                    source.pointers[source_c_idx]
                        .offset(source_component.size as isize * source_index as isize),
                    destination.pointers[destination_c_idx]
                        .offset(destination_component.size as isize * destination_index as isize),
                    source_component.size as usize,
                );
            }
        }
    }

    /// Returns the pointers for the components in [`G`], provided that archetype itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - offset must be smaller than self.capacity.
    /// - Only call this with subsets of the types stored in the shard.
    unsafe fn get_fuzzy_pointers_unchecked<'a, G: ComponentGroup>(
        &'a self,
        offset: u32,
    ) -> [*mut u8; MAX_COMPONENTS_PER_ENTITY] {
        let mut pointers = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        for (index, descriptor) in G::DESCRIPTOR.archetype().components().iter().enumerate() {
            'inner_loop: for check_index in index..self.descriptor.len() as usize {
                if self
                    .descriptor
                    .components_unchecked()
                    .get_unchecked(check_index)
                    .component_type_id
                    .into_u16()
                    == descriptor.component_type_id.into_u16()
                {
                    *pointers.get_unchecked_mut(index) = self
                        .pointers
                        .get_unchecked(check_index)
                        .offset(descriptor.size() as isize * offset as isize);
                    break 'inner_loop;
                }
            }
        }
        pointers
    }
}
