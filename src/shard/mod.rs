mod metadata;
#[cfg(test)]
mod tests;

use crate::archetype::*;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_group::ComponentGroup;
use crate::constants::*;
use crate::entity::*;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::hint::unreachable_unchecked;
use core::num::NonZeroU16;
use core::ptr::null_mut;
use metadata::*;

#[derive(Debug)]
pub(crate) struct Shard {
    components: [*mut u8; MAX_COMPONENTS_PER_ENTITY],
    entities: Option<Box<[EntityMetadata; ENTITIES_PER_SHARD]>>,
    entity_count: u16,
    archetype_index: u16,
    next_shard: u16, // Forms a linked list!
                     //reorder_buffer_handle: u16, // TODO: re-order buffer handle.
}

impl Shard {
    pub fn archetype_index(&self) -> u16 {
        self.archetype_index
    }

    pub fn next_shard(&self) -> u16 {
        self.next_shard
    }

    pub fn set_next_shard(&mut self, next_shard: u16) {
        self.next_shard = next_shard
    }

    /// Returns true if the shard is in recycled state.
    /// Recycled state means that it contains no valid entity data.
    /// It is indicated by whether the entities array is present or not.
    #[cfg(debug_assertions)]
    pub fn is_recycled_shard(&self) -> bool {
        self.entities.is_none()
    }

    /// Drops the components in the shard and deallocates the memory. This is unsafe for obvious reasons!
    pub unsafe fn drop_and_dealloc_components(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) {
        if self.entity_count == 0 {
            return;
        }
        for i in 0..archetype_descriptor.len() as usize {
            (archetype_descriptor.components()[i].fns.drop_handler)(
                self.components[i],
                self.entity_count as usize,
            );
        }
        Self::dealloc_pointers(&mut self.components, archetype_descriptor);
        self.entity_count = 0;
    }

    pub unsafe fn recycle(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Box<[EntityMetadata; ENTITIES_PER_SHARD]> {
        debug_assert!(self.entities.is_some());

        self.drop_and_dealloc_components(archetype_descriptor);
        self.entities.take().unwrap()
    }

    /// Constructs a new shard instance using the descriptor and archetype index.
    /// TODO: Allow re-use of the entity metadata array.
    pub fn new(archetype: &ArchetypeDescriptor, archetype_index: u16) -> Option<Self> {
        use alloc::alloc::*;
        let mut ptrs = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        for i in 0..archetype.len() as usize {
            unsafe {
                let layout = Layout::from_size_align_unchecked(
                    archetype.components()[i].size() as usize * MAX_COMPONENTS_PER_ENTITY,
                    archetype.components()[i].align() as usize,
                );
                ptrs[i] = alloc(layout);
                // Check for alloc failures.
                if ptrs[i] == core::ptr::null_mut() {
                    Self::dealloc_pointers(&mut ptrs[0..i], archetype);
                    return None;
                }
            }
        }
        Self {
            components: ptrs,
            entities: Box::new([Default::default(); ENTITIES_PER_SHARD]).into(),
            entity_count: 0,
            archetype_index,
            next_shard: INVALID_SHARD_INDEX,
            //reorder_buffer_handle: 0,
        }
        .into()
    }

    /// Pushes an entity into the end of the shard.
    /// # Safety
    /// - MUST only be called with a type G where G is identical to the type contained in the shard pool.
    /// - G must be 'equavalent': It must be a group containing the same components as contained in the shard.
    /// - But ordering of these components may be different.
    /// - Assumes length of the pool is checked beforehand.
    /// - Takes ownership of `entity`, not calling drop of couse.
    /// - Assumes that there is enough space in the shard.
    pub unsafe fn push_entity_unchecked<'s, G: ComponentGroup<'s>>(
        &mut self,
        metadata: EntityMetadata,
        entity: G,
    ) -> u16 {
        let current_idx = self.entity_count;
        self.entity_count += 1;
        self.write_entity_unchecked(current_idx, metadata, entity);
        current_idx
    }

    /// Writes a given entity/component-tuple to the shard's backing memory.
    /// # Safety
    /// - MUST only be called with a type G where G is identical to the type contained in the archetype pool.
    /// - G must be 'equivalent': It must be a group containing the same components as contained in the pool.
    /// - But ordering of these components may be different.
    /// - Does NOT call drop on the given entity.
    /// - Does NOT call drop on the internal memory, so this needs to be correctly handled manually!.
    pub(crate) unsafe fn write_entity_unchecked<'s, G: ComponentGroup<'s>>(
        &mut self,
        index: u16,
        metadata: EntityMetadata,
        mut entity: G,
    ) -> u16 {
        let mut pointers = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        entity.as_sorted_pointers(&mut pointers);
        for i in 0..G::DESCRIPTOR.len() as usize {
            core::ptr::copy_nonoverlapping(
                pointers[i],
                self.components[i],
                G::DESCRIPTOR.archetype().components()[i].size as usize,
            );
        }
        self.entities.as_mut().unwrap()[index as usize] = metadata;
        core::mem::forget(entity);
        index
    }

    /// Returns a tuple as component slices to the shard's contents.
    /// # Safety:
    /// - MUST be exactly the component group it's allocated with.
    /// - A compatible one is also fine.
    #[inline(always)]
    pub unsafe fn get_slices_unchecked_exact<'s, G: ComponentGroup<'s>>(
        &'s self,
    ) -> G::SliceRefTuple {
        G::slice_unchecked(&self.components, self.entity_count as usize)
    }

    /// Returns a tuple as mutable component slices to the shard's contents.
    /// # Safety:
    /// - MUST be exactly the component group it's allocated with.
    /// - A compatible one is also fine.
    #[inline(always)]
    pub unsafe fn get_slices_unchecked_exact_mut<'s, G: ComponentGroup<'s>>(
        &'s mut self,
    ) -> G::SliceMutRefTuple {
        G::slice_unchecked_mut(&self.components, self.entity_count as usize)
    }

    #[inline(always)]
    unsafe fn get_fuzzy_pointers_unchecked<'s, G: ComponentGroup<'s>>(
        &'s self,
        archetype: &ArchetypeDescriptor,
    ) -> [*mut u8; MAX_COMPONENTS_PER_ENTITY] {
        let mut pointers = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        for (index, descriptor) in G::DESCRIPTOR.archetype().components().iter().enumerate() {
            'inner_loop: for check_index in index..archetype.len() as usize {
                if archetype.components()[check_index]
                    .component_type_id
                    .into_u16()
                    == descriptor.component_type_id.into_u16()
                {
                    pointers[index] = self.components[check_index];
                    break 'inner_loop;
                }
            }
        }
        pointers
    }

    /// Returns the slices for the components in G, provided that shard itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the shard.
    pub unsafe fn get_fuzzy_slices_unchecked<'s, G: ComponentGroup<'s>>(
        &'s self,
        archetype: &ArchetypeDescriptor,
    ) -> G::SliceRefTuple {
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(&archetype);
        G::slice_unchecked(&pointers, self.entity_count as usize)
    }

    /// Returns the mutable slices for the components in G, provided that shard itself contains a superset of G.
    /// This function is slower than the exact version, use that if an exact type match is known.
    /// # Safety:
    /// - Only call this with subsets of the types stored in the shard.
    pub unsafe fn get_fuzzy_slices_unchecked_mut<'s, G: ComponentGroup<'s>>(
        &'s mut self,
        archetype: &ArchetypeDescriptor,
    ) -> G::SliceMutRefTuple {
        let pointers = self.get_fuzzy_pointers_unchecked::<G>(&archetype);
        G::slice_unchecked_mut(&pointers, self.entity_count as usize)
    }

    pub fn is_full(&self) -> bool {
        self.entity_count as usize >= ENTITIES_PER_SHARD
    }

    /// Forcibly deallocates the memory associated with the pointers.
    /// # Safety:
    /// - Deallocated the backing memory of the shard.
    /// - Components must be dropped beforehand!
    /// - Must only be called with the descriptor it was allocated with!
    unsafe fn dealloc_pointers(pointers: &mut [*mut u8], descriptor: &ArchetypeDescriptor) {
        use alloc::alloc::*;
        for i in 0..pointers.len() {
            let layout = Layout::from_size_align_unchecked(
                descriptor.components()[i].size() as usize * MAX_COMPONENTS_PER_ENTITY,
                descriptor.components()[i].align() as usize,
            );
            dealloc(pointers[i], layout);
            pointers[i] = core::ptr::null_mut();
        }
    }
}
