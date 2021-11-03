use crate::archetype::*;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_group::ComponentGroup;
use crate::constants::*;
use crate::entity::*;
use alloc::boxed::Box;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct EntityMetadata {
    value: Entity,
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self {
            value: Entity::INVALID,
        }
    }
}

impl EntityMetadata {
    /// Get a reference to the entity metadata's value.
    pub(crate) fn entity(&self) -> Entity {
        unsafe { Entity::new(self.value.index(), 0) }
    }
}

#[derive(Debug)]
pub(crate) struct Shard {
    components: [*mut u8; MAX_COMPONENTS_PER_ENTITY],
    entities: Box<[EntityMetadata; ENTITIES_PER_SHARD]>,
    entity_count: u16,
    next_shard: Option<u16>,
    archetype_index: u16,
}

impl Shard {
    pub fn alloc(archetype: &ArchetypeDescriptor, arch_index: u16) -> Option<Self> {
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
                    for j in 0..i {
                        let layout = Layout::from_size_align_unchecked(
                            archetype.components()[j].size() as usize * MAX_COMPONENTS_PER_ENTITY,
                            archetype.components()[j].align() as usize,
                        );
                        dealloc(ptrs[j], layout)
                    }
                    return None;
                }
            }
        }
        Self {
            components: ptrs,
            entities: Box::new([Default::default(); 3072]),
            entity_count: 0,
            next_shard: None,
            archetype_index: arch_index,
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
        self.entities[index as usize] = metadata;
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
            'inner_loop: for check_index in index..G::DESCRIPTOR.len() as usize {
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

    pub fn has_next(&self) -> Option<u16> {
        self.next_shard
    }

    pub fn is_full(&self) -> bool {
        self.entity_count as usize >= ENTITIES_PER_SHARD
    }

    /// Get the shard's archetype index.
    pub(crate) fn archetype_index(&self) -> u16 {
        self.archetype_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn test_shard() {
        unsafe {
            let group = (A::default(), B::default(), C::default());
            let descriptor = <(A, B, C) as ComponentGroup<'_>>::DESCRIPTOR.archetype();

            let mut shard = Shard::alloc(&descriptor, 0);
            assert!(shard.is_some());
            let mut shard = shard.unwrap();

            let meta = EntityMetadata {
                value: Default::default(),
            };
            let idx = shard.push_entity_unchecked(meta, (A::default(), B::default(), C::default()));
            assert_eq!(shard.entity_count, 1);
            assert_eq!(meta, shard.entities[0]);

            let slices: (&[A], &[B], &[C]) = shard.get_slices_unchecked_exact::<(A, B, C)>();
            assert_eq!(slices.0.len(), 1);
            assert_eq!(slices.1.len(), 1);
            assert_eq!(slices.0[0], A::default());
            assert_eq!(slices.1[0], B::default());
            assert_eq!(slices.2[0], C::default());
            let slices: (&[B], &[A], &[C]) = shard.get_slices_unchecked_exact::<(B, A, C)>();
            assert_eq!(slices.0.len(), 1);
            assert_eq!(slices.1.len(), 1);
            assert_eq!(slices.0[0], B::default());
            assert_eq!(slices.1[0], A::default());
            assert_eq!(slices.2[0], C::default());

            let slices: (&[B], &[A]) = shard.get_fuzzy_slices_unchecked::<(B, A)>(descriptor);
            assert_eq!(slices.0.len(), 1);
            assert_eq!(slices.1.len(), 1);
            assert_eq!(slices.0[0], B::default());
            assert_eq!(slices.1[0], A::default());

            let slices: (&[A], &[C]) = shard.get_fuzzy_slices_unchecked::<(A, C)>(descriptor);
            assert_eq!(slices.0.len(), 1);
            assert_eq!(slices.1.len(), 1);
            assert_eq!(slices.0[0], A::default());
            assert_eq!(slices.1[0], C::default());
        }
    }
}
