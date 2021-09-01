use alloc::{boxed::Box, vec::Vec};

use crate::{
    archetype_descriptor::ArchetypeDescriptor, component_descriptor::ComponentDescriptor,
    ArchetypeId, Component, Entity, ENTITIES_PER_SHARD, MAX_COMPONENTS_PER_ENTITY,
};

#[derive(Debug)]
pub(crate) struct Shard {
    components: [*mut u8; MAX_COMPONENTS_PER_ENTITY],
    entities: Option<Box<[Entity; ENTITIES_PER_SHARD]>>,
    entity_count: u16,
    next_shard: u16,
    archetype: u16,
}

impl Shard {
    pub fn alloc(archetype: &Archetype, arch_index: u16) -> Option<Self> {
        use alloc::alloc::*;
        let mut ptrs = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
        for i in 0..archetype.descriptor.len() as usize {
            unsafe {
                let layout = Layout::from_size_align_unchecked(
                    archetype.descriptor.components()[i].size() as usize
                        * MAX_COMPONENTS_PER_ENTITY,
                    archetype.descriptor.components()[i].align() as usize,
                );
                ptrs[i] = alloc(layout);
                // Check for alloc failures.
                if ptrs[i] == core::ptr::null_mut() {
                    for j in 0..i {
                        let layout = Layout::from_size_align_unchecked(
                            archetype.descriptor.components()[j].size() as usize
                                * MAX_COMPONENTS_PER_ENTITY,
                            archetype.descriptor.components()[j].align() as usize,
                        );
                        dealloc(ptrs[j], layout)
                    }
                    return None;
                }
            }
        }
        Self {
            components: ptrs,
            entities: Some(Box::new([Default::default(); 3072])),
            entity_count: 0,
            next_shard: u16::MAX,
            archetype: arch_index,
        }
        .into()
    }

    pub fn has_next(&self) -> Option<u16> {
        match self.next_shard {
            u16::MAX => None,
            v => Some(v),
        }
    }

    pub fn is_full(&self) -> bool {
        self.entity_count as usize == ENTITIES_PER_SHARD
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SortedArchetypeKey {
    id: ArchetypeId,
    archetype_index: u16,
}

pub(crate) struct AddRemoveInfo<'a> {
    current_archetype: &'a mut Archetype,
    current_shard: &'a mut Shard,
    new_archetype: &'a mut Archetype,
    new_shard: &'a mut Shard,
}

#[derive(Debug, Clone)]
pub(crate) struct Archetype {
    descriptor: ArchetypeDescriptor,
    shard_indices: Vec<u16>,
    super_sets: Vec<u16>,
    sub_sets: Vec<u16>,
}

pub(crate) struct ShardRegistry {
    sorted_mappings: [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
    archetypes: [Vec<Archetype>; MAX_COMPONENTS_PER_ENTITY],
    shards: Vec<Shard>,
}

impl Default for ShardRegistry {
    fn default() -> Self {
        Self {
            shards: Vec::with_capacity(2048),
            archetypes: [
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
            ],
            sorted_mappings: [
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
            ],
        }
    }
}

impl ShardRegistry {
    /// New archetypes can, for now atleast, only be instantiated from 1 component only!
    pub fn get_or_alloc_shard_from_component_descriptor(
        &mut self,
        component: &ComponentDescriptor,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        match self.sorted_mappings[0]
            .binary_search_by_key(&component.component_type_id().into_u16(), |e| {
                e.id.into_u32() as u16
            }) {
            Ok(arch_index) => {
                let archetype = &mut self.archetypes[0][arch_index];
                if archetype.shard_indices.is_empty()
                    || self.shards[*archetype.shard_indices.last().unwrap() as usize].is_full()
                {
                    // alloc new shard.
                    let shard_idx = self.shards.len();
                    self.shards
                        .push(Shard::alloc(archetype, arch_index as u16)?);
                    archetype.shard_indices.push(shard_idx as u16);
                    return Some((archetype, self.shards.last_mut().unwrap()));
                } else {
                    let last_shard =
                        &mut self.shards[*archetype.shard_indices.last().unwrap() as usize];
                    return Some((archetype, last_shard));
                }
            }
            Err(insertion_index) => {
                let mut archetype = Archetype {
                    descriptor: component.into(),
                    shard_indices: Vec::with_capacity(8),
                    super_sets: Vec::with_capacity(16),
                    sub_sets: Vec::with_capacity(0),
                };
                // alloc new shard.
                let shard_idx = self.shards.len();
                let arch_index = self.archetypes[0].len();
                self.shards
                    .push(Shard::alloc(&archetype, arch_index as u16)?);
                archetype.shard_indices.push(shard_idx as u16);
                self.archetypes[0].push(archetype);
                self.sorted_mappings[0].insert(
                    insertion_index,
                    SortedArchetypeKey {
                        id: component.component_type_id().into(),
                        archetype_index: arch_index as u16,
                    },
                );
                let archetype = self.archetypes[0].last_mut().unwrap();
                let last_shard =
                    &mut self.shards[*archetype.shard_indices.last().unwrap() as usize];
                return Some((archetype, last_shard));
            }
        }
    }

    pub(crate) fn get_or_alloc_shard_adding_component<C: Component>(
        &mut self,
        current_shard_index: u16,
        current_archetype_len: u8,
    ) -> Option<AddRemoveInfo<'_>> {
        let current_shard = self.shards.get_mut(current_shard_index as usize)?;
        let current_archetype = self
            .archetypes
            .get_mut(current_archetype_len as usize)?
            .get_mut(current_shard.archetype as usize)?;

        let new_archetype = current_archetype.descriptor.add_component::<C>()?;

        None
    }
}
