use crate::archetype::*;
use crate::component_group::ComponentGroup;
use crate::shard::*;
use crate::{
    archetype_descriptor::ArchetypeDescriptor, component_descriptor::ComponentDescriptor,
    ArchetypeId, Component, Entity, ENTITIES_PER_SHARD, MAX_COMPONENTS_PER_ENTITY,
};
use alloc::vec;
use alloc::{boxed::Box, vec::Vec};

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
            ],
        }
    }
}

impl ShardRegistry {
    pub fn get_or_alloc_shard_from_group<'s, G: ComponentGroup<'s>>(
        &mut self,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        debug_assert!(G::DESCRIPTOR.is_some());
        debug_assert!(G::DESCRIPTOR.unwrap().len() != 0);

        let archetypes_index = G::DESCRIPTOR.unwrap().len() as usize - 1;
        return match self.sorted_mappings[G::DESCRIPTOR.unwrap().len() as usize - 1]
            .binary_search_by_key(&G::DESCRIPTOR.unwrap().archetype().archetype_id(), |e| e.id)
        {
            Ok(archetype_index) => {
                let archetype = &mut self.archetypes[archetypes_index][archetype_index];

                if archetype.shard_indices_mut().is_empty()
                    || self.shards[*archetype.shard_indices_mut().last().unwrap() as usize]
                        .is_full()
                {
                    // alloc new shard.
                    let shard_idx = self.shards.len();
                    self.shards.push(Shard::alloc(
                        archetype.descriptor(),
                        archetype_index as u16,
                    )?);
                    archetype.shard_indices_mut().push(shard_idx as u16);
                    Some((archetype, self.shards.last_mut().unwrap()))
                } else {
                    let last_shard =
                        &mut self.shards[*archetype.shard_indices_mut().last().unwrap() as usize];
                    Some((archetype, last_shard))
                }
            }
            Err(insertion_index) => {
                let mut archetype = Archetype::new(
                    G::DESCRIPTOR.unwrap().archetype().clone(),
                    Vec::with_capacity(8),
                );
                // alloc new shard.
                let shard_idx = self.shards.len();
                let arch_index = self.archetypes[archetypes_index].len();
                self.shards
                    .push(Shard::alloc(&archetype.descriptor(), arch_index as u16)?);
                archetype.shard_indices_mut().push(shard_idx as u16);
                self.archetypes[archetypes_index].push(archetype);
                self.sorted_mappings[archetypes_index].insert(
                    insertion_index,
                    SortedArchetypeKey {
                        id: G::DESCRIPTOR.unwrap().archetype().archetype_id(),
                        archetype_index: arch_index as u16,
                    },
                );
                let archetype = self.archetypes[archetypes_index].last_mut().unwrap();
                let last_shard =
                    &mut self.shards[*archetype.shard_indices_mut().last().unwrap() as usize];
                Some((archetype, last_shard))
            }
        };
    }

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
                if archetype.shard_indices_mut().is_empty()
                    || self.shards[*archetype.shard_indices_mut().last().unwrap() as usize]
                        .is_full()
                {
                    // alloc new shard.
                    let shard_idx = self.shards.len();
                    self.shards
                        .push(Shard::alloc(archetype.descriptor(), arch_index as u16)?);
                    archetype.shard_indices_mut().push(shard_idx as u16);
                    return Some((archetype, self.shards.last_mut().unwrap()));
                } else {
                    let last_shard =
                        &mut self.shards[*archetype.shard_indices_mut().last().unwrap() as usize];
                    return Some((archetype, last_shard));
                }
            }
            Err(insertion_index) => {
                let mut archetype = Archetype::new(component.into(), Vec::with_capacity(8));
                // alloc new shard.
                let shard_idx = self.shards.len();
                let arch_index = self.archetypes[0].len();
                self.shards
                    .push(Shard::alloc(&archetype.descriptor(), arch_index as u16)?);
                archetype.shard_indices_mut().push(shard_idx as u16);
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
                    &mut self.shards[*archetype.shard_indices_mut().last().unwrap() as usize];
                return Some((archetype, last_shard));
            }
        }
    }

    // pub(crate) fn get_or_alloc_shard_adding_component<C: Component>(
    //     &mut self,
    //     current_shard_index: u16,
    //     current_archetype_len: u8,
    // ) -> Option<AddRemoveInfo<'_>> {
    //     let current_shard = self.shards.get_mut(current_shard_index as usize)?;
    //     let current_archetype = self
    //         .archetypes
    //         .get_mut(current_archetype_len as usize)?
    //         .get_mut(current_shard.archetype_index() as usize)?;

    //     let new_archetype = current_archetype.descriptor().add_component::<C>()?;

    //     None
    // }
}
