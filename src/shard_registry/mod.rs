#[cfg(test)]
mod tests;

mod internal;
mod sorted_archetype_key;

use internal::*;
use sorted_archetype_key::*;

use crate::archetype::*;
use crate::component_group::ComponentGroup;
use crate::shard::*;
use crate::{
    archetype_descriptor::ArchetypeDescriptor, component_descriptor::ComponentDescriptor,
    ArchetypeId, Component, Entity, ENTITIES_PER_SHARD, MAX_ARCHETYPE_COUNT,
    MAX_COMPONENTS_PER_ENTITY, MAX_SHARD_COUNT,
};
use alloc::vec;
use alloc::{boxed::Box, vec::Vec};

pub(crate) struct ShardRegistry {
    sorted_mappings: [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
    archetypes: [Vec<Archetype>; MAX_COMPONENTS_PER_ENTITY],
    shards: Vec<Shard>,
    next_recyclable_shard: Option<u16>,
}

impl ShardRegistry {
    #[inline(always)]
    pub fn find_or_create_single_entity_shard_from_group<'s, G: ComponentGroup<'s>>(
        &mut self,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        self.find_or_create_single_entity_shard(G::DESCRIPTOR.archetype())
    }

    pub fn find_or_create_single_entity_shard(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        // Check validity of the archetype.
        if !archetype_descriptor.is_valid() {
            return None;
        }
        let archetype_level_index = archetype_descriptor.len() as usize - 1;
        debug_assert!(archetype_level_index < MAX_COMPONENTS_PER_ENTITY);

        // We need to binary search the sort keys to check if our archetype exists.
        return match self.sorted_mappings[archetype_level_index]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => self.find_or_create_single_entity_shard_for_archetype(
                archetype_level_index,
                found_index,
            ),
            Err(insertion_index) => {
                // Archetype not found, we need to create a new one.
                self.create_archetype_and_shard(archetype_descriptor, insertion_index as u16)
            }
        };
    }
}
