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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SortedArchetypeKey {
    id: ArchetypeId,
    archetype_index: u16,
}

pub(crate) struct ShardRegistry {
    sorted_mappings: [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
    archetypes: [Vec<Archetype>; MAX_COMPONENTS_PER_ENTITY],
    shards: Vec<Shard>,
    next_recyclable_shard: Option<u16>,
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
            next_recyclable_shard: None,
        }
    }
}

impl ShardRegistry {
    pub fn find_or_create_non_empty_archetype_shard(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        if !archetype_descriptor.is_valid() {
            // Effectively this ensures length is valid ( 0 < len <= MAX_COMPONENTS_PER_ENTITY by construction.
            return None;
        }
        let archetype_level_index = archetype_descriptor.len() as usize - 1;
        return match self.sorted_mappings[archetype_level_index]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => {
                let archetype_key = &self.sorted_mappings[archetype_level_index][found_index];
                let mut archetype = &mut self.archetypes[archetype_level_index]
                    [archetype_key.archetype_index as usize];

                unimplemented!()
            }
            Err(insertion_index) => {
                // Archetype not found
                self.create_archetype_and_shard(archetype_descriptor, insertion_index as u16)
            }
        };
    }

    unsafe fn fetch_or_create_shard(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
        archetype_index: u16,
    ) -> Option<(&mut Shard, u16)> {
        return if let Some(shard_index) = self.next_recyclable_shard {
            let recyclable_shard = &mut self.shards[shard_index as usize];
            //TODO: Implement reuse?
            // In theory if the archetype is identical we might not need to re_alloc.
            // We might also be able to re-use part of the allocations.
            // For now assume it is deallocated already.
            let next_recyclable_shard_index = recyclable_shard.has_next();
            let mut new_shard = Shard::new(&archetype_descriptor, archetype_index)?;
            //TODO: We could do without this and do it in place.
            core::mem::swap(recyclable_shard, &mut new_shard);
            self.next_recyclable_shard = next_recyclable_shard_index;
            Some((recyclable_shard, shard_index))
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

#[cfg(test)]
mod tests {
    use crate::shard_registry::ShardRegistry;

    #[test]
    fn test_create_default_shard_registry() {
        ShardRegistry::default();
    }
}
