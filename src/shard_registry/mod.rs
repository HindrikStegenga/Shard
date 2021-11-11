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

    fn find_or_create_single_entity_shard_for_archetype(
        &mut self,
        archetype_level_index: usize,
        sort_key_index: usize,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        let mut archetype = {
            // found_index is the index of the sort key, find the archetype index using the sort key.
            let archetype_index =
                self.sorted_mappings[archetype_level_index][sort_key_index].archetype_index;
            // Grab the archetype using the sort key.
            &mut self.archetypes[archetype_level_index][archetype_index as usize]
        };

        // If the last shard still has space for an entity, we can return this shard and the archetype.
        let last_shard_index = archetype.last_shard_index() as usize;
        if !self.shards[last_shard_index].is_full() {
            return Some((archetype, &mut self.shards[last_shard_index]));
        }
        // We need to recycle or create a new shard.
        Self::create_or_recycle_shard(archetype, &mut self.shards, &mut self.next_recyclable_shard)
    }

    fn create_or_recycle_shard<'a>(
        archetype: &'a mut Archetype,
        shards: &'a mut Vec<Shard>,
        next_recyclable_shard: &'a mut Option<u16>,
    ) -> Option<(&'a mut Archetype, &'a mut Shard)> {
        return if let Some(shard_index) = next_recyclable_shard {
            let mut last_shard = &mut shards[archetype.last_shard_index() as usize];
            // Set the new last shard index on the current last shard..
            //last_shard.set_next_shard(Some(*shard_index));

            unimplemented!()
        } else {
            unimplemented!()
        };
    }

    unsafe fn fetch_or_create_shard(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
        archetype_index: u16,
    ) -> Option<(&mut Shard, u16)> {
        let mut archetype =
            &mut self.archetypes[archetype_descriptor.len() as usize - 1][archetype_index as usize];
        return if let Some(shard_index) = self.next_recyclable_shard {
            let mut last_shard = &mut self.shards[archetype.last_shard_index() as usize];
            //last_shard.set_next_shard(Some(shard_index));
            archetype.set_last_shard_index(shard_index);
            let recyclable_shard = &mut self.shards[shard_index as usize];
            //TODO: Implement reuse?
            // In theory if the archetype is identical we might not need to re_alloc.
            // We might also be able to re-use part of the allocations.
            // For now assume it is deallocated already.
            //let next_recyclable_shard_index = recyclable_shard.has_next();
            //let mut new_shard = Shard::new(&archetype_descriptor, archetype_index)?;
            //TODO: We could do without this and do it in place.
            //core::mem::swap(recyclable_shard, &mut new_shard);
            //self.next_recyclable_shard = next_recyclable_shard_index;
            unimplemented!()
            //Some((recyclable_shard, shard_index))
        } else {
            if self.shards.len() >= MAX_SHARD_COUNT {
                return None;
            }
            // Push a new shard onto the back of the registry.
            let new_shard_index = self.shards.len() as u16;
            let shard = Shard::new(&archetype_descriptor, archetype_index)?;
            self.shards.push(shard);
            Some((self.shards.last_mut().unwrap(), new_shard_index))
        };
    }

    fn create_archetype_and_shard(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
        insertion_index: u16,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        if self.shards.len() >= MAX_SHARD_COUNT || self.archetypes.len() >= MAX_ARCHETYPE_COUNT {
            return None;
        }
        debug_assert!(archetype_descriptor.is_valid());

        let new_arch_index = (archetype_descriptor.len() as usize - 1) as u16;
        let (_, new_shard_index) =
            unsafe { self.fetch_or_create_shard(&archetype_descriptor, new_arch_index)? };
        let archetype = Archetype::new(archetype_descriptor.clone(), new_shard_index as u16);

        self.sorted_mappings[archetype.descriptor().len() as usize - 1].insert(
            insertion_index as usize,
            SortedArchetypeKey {
                id: archetype.descriptor().archetype_id(),
                archetype_index: new_arch_index,
            },
        );
        self.archetypes[new_arch_index as usize].push(archetype);
        Some((
            self.archetypes[new_arch_index as usize].last_mut().unwrap(),
            &mut self.shards[new_shard_index as usize],
        ))
    }
}
