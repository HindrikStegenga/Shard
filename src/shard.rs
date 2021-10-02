use crate::archetype::*;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_group::ComponentGroup;
use crate::constants::*;
use crate::entity::*;
use alloc::boxed::Box;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct EntityMetadata {
    value: Entity,
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self { value: Entity::INVALID }
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
    /// - Takes ownership of `entity`, not calling drop ofcouse.
    pub unsafe fn push_entity_unchecked<G: ComponentGroup>(
        &mut self,
        metadata: EntityMetadata,
        entity: G,
    ) -> u16 {
        todo!()
    }

    /// Writes a given entity/component-tuple to the shard's backing memory.
    /// # Safety
    /// - MUST only be called with a type G where G is identical to the type contained in the archetype pool.
    /// - G must be 'equavalent': It must be a group containing the same components as contained in the pool.
    /// - But ordering of these components may be different.
    /// - Does NOT call drop on the given entity.
    /// - Does NOT call drop on the internal memory, so this needs to be correctly handled manually!.
    pub(crate) unsafe fn write_entity_unchecked<G: ComponentGroup>(
        &mut self,
        index: u16,
        metadata: EntityMetadata,
        mut entity: G,
    ) -> u16 {
        for i in 0..G::LENGTH as usize {
            core::ptr::copy_nonoverlapping(
                self.components[i],
                self.components[i],
                G::SORTED_DESCRIPTORS[i].size as usize,
            );
        }
        self.entities[index as usize] = metadata;
        core::mem::forget(entity);
        index
    }

    pub fn has_next(&self) -> Option<u16> {
        self.next_shard
    }

    pub fn is_full(&self) -> bool {
        self.entity_count as usize == ENTITIES_PER_SHARD
    }

    /// Get the shard's archetype index.
    pub(crate) fn archetype_index(&self) -> u16 {
        self.archetype_index
    }
}
