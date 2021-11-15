use super::*;
use crate::INVALID_SHARD_INDEX;

impl Default for ShardRegistry {
    fn default() -> Self {
        Self {
            shards: Vec::with_capacity(256),
            archetypes: [
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
            ],
            sorted_mappings: [
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
            ],
            next_recyclable_shard: None,
        }
    }
}

impl ShardRegistry {
    #[cfg(debug_assertions)]
    pub(super) fn verify_consistency(&self) -> bool {
        self.archetypes.iter().for_each(|e| {
            e.iter().enumerate().for_each(|(idx, e)| {
                assert_ne!(e.first_shard_index(), 0);
                let mut current_sh_idx = e.first_shard_index();
                while {
                    let shard = &self.shards[current_sh_idx as usize];
                    assert!(shard.is_recycled_shard());
                    assert_eq!(shard.archetype_index() as usize, idx);
                    current_sh_idx = shard.next_shard();
                    current_sh_idx != e.last_shard_index()
                } {}
            });
        });
        return true;
    }

    pub(super) fn find_or_create_single_entity_shard_for_archetype(
        &mut self,
        archetype_level_index: usize,
        sort_key_index: usize,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        debug_assert!(self.verify_consistency());
        let archetype_index =
            self.sorted_mappings[archetype_level_index][sort_key_index].archetype_index;
        let mut archetype = &mut self.archetypes[archetype_level_index][archetype_index as usize];

        // If the last shard still has space for an entity, we can return this shard and the archetype.
        let last_shard_index = archetype.last_shard_index() as usize;
        if !self.shards[last_shard_index].is_full() {
            return Some((archetype, &mut self.shards[last_shard_index]));
        }
        // We need to recycle or create a new shard.
        let (a, s, _) = Self::create_or_recycle_shard_for_archetype(
            archetype,
            archetype_index,
            &mut self.shards,
            &mut self.next_recyclable_shard,
        )?;
        Some((a, s))
    }

    fn create_or_recycle_shard_for_archetype<'a>(
        archetype: &'a mut Archetype,
        archetype_index: u16,
        shards: &'a mut Vec<Shard>,
        next_recyclable_shard: &'a mut Option<u16>,
    ) -> Option<(&'a mut Archetype, &'a mut Shard, u16)> {
        let last_shard_index = archetype.last_shard_index() as usize;
        debug_assert!(!shards[last_shard_index].is_recycled_shard());

        return if let Some(shard_index) = next_recyclable_shard {
            let mut last_shard = &mut shards[last_shard_index];
            // Set the new last shard index on the current last shard..
            //last_shard.set_next_shard(Some(*shard_index));

            unimplemented!()
        } else {
            // No shard to recycle, so we create a new shard.

            let shard_index = shards.len() as u16;
            shards.push(Shard::new(archetype.descriptor(), archetype_index)?);
            debug_assert_eq!(shards[last_shard_index].next_shard(), INVALID_SHARD_INDEX);
            shards[last_shard_index].set_next_shard(shard_index);
            archetype.set_last_shard_index(shard_index);
            Some((archetype, &mut shards[shard_index as usize], shard_index))
        };
    }

    pub(super) fn create_archetype_and_shard(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
        insertion_index: u16,
    ) -> Option<(&mut Archetype, &mut Shard)> {
        if self.shards.len() >= MAX_SHARD_COUNT as usize
            || self.archetypes.len() >= MAX_ARCHETYPE_COUNT
        {
            return None;
        }
        debug_assert!(archetype_descriptor.is_valid());

        let new_arch_index = (archetype_descriptor.len() as usize - 1) as u16;
        let mut archetype = Archetype::new(archetype_descriptor.clone(), 0);
        let new_shard_index = 0;
        //archetype.set_last_shard_index(shard_index);
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
