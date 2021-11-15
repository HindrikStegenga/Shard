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
                let mut current_sh_idx = e.first_shard_index();
                while {
                    let shard = &self.shards[current_sh_idx as usize];
                    assert!(shard.is_recyclable_shard());
                    assert_eq!(shard.archetype_index() as usize, idx);
                    current_sh_idx = shard.next_shard();
                    current_sh_idx != e.last_shard_index() && current_sh_idx != INVALID_SHARD_INDEX
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
        mut next_recyclable_shard: &'a mut Option<u16>,
    ) -> Option<(&'a mut Archetype, &'a mut Shard, u16)> {
        let last_shard_index = archetype.last_shard_index() as usize;
        debug_assert!({ if last_shard_index != INVALID_SHARD_INDEX as usize { !shards[last_shard_index].is_recyclable_shard() } else { true }});

        return if let Some(shard_index) = *next_recyclable_shard {
            let mut shard = &mut shards[shard_index as usize];
            debug_assert!(shard.is_recyclable_shard());
            // Store next recyclable shard in linked list.
            let mut t_recycle_shard = {
                let v = shard.next_shard();
                if v == INVALID_SHARD_INDEX {
                    None
                } else {
                    Some(v)
                }
            };
            let mut shard = unsafe { shard.recycle(archetype.descriptor(), archetype_index)? };
            // Update next recyclable linked list
            *next_recyclable_shard = t_recycle_shard;
            shard.set_next_shard(INVALID_SHARD_INDEX);
            if last_shard_index != INVALID_SHARD_INDEX as usize {
                shards[last_shard_index].set_next_shard(shard_index);
            }
            archetype.set_last_shard_index(shard_index);
            Some((archetype, &mut shards[shard_index as usize], shard_index))
        } else {
            // No shard to recycle, so we create a new shard.

            let shard_index = shards.len() as u16;
            shards.push(Shard::new(archetype.descriptor(), archetype_index)?);
            debug_assert_eq!({ if last_shard_index != INVALID_SHARD_INDEX as usize { shards[last_shard_index].next_shard() } else { INVALID_SHARD_INDEX }}, INVALID_SHARD_INDEX);

            if last_shard_index != INVALID_SHARD_INDEX as usize {
                shards[last_shard_index].set_next_shard(shard_index);
            }

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
        let mut archetype = Archetype::new(archetype_descriptor.clone(), INVALID_SHARD_INDEX);
        let (_, _, new_shard_index) = Self::create_or_recycle_shard_for_archetype(&mut archetype, new_arch_index, &mut self.shards, &mut self.next_recyclable_shard)?;
        unsafe { archetype.set_first_shard_index(new_shard_index) };
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

impl Drop for ShardRegistry {
    fn drop(&mut self) {
        //TODO: Consider storing a compressed level index into shards?
        // It could be stored in entities? it would make this dropping more efficient.
        self.archetypes.iter().for_each(|e| {
            e.iter().enumerate().for_each(|(idx, e)| {
                let mut current_sh_idx = e.first_shard_index();
                while {
                    let shard = &mut self.shards[current_sh_idx as usize];
                    unsafe { shard.drop_and_dealloc_components(e.descriptor()) };
                    current_sh_idx = shard.next_shard();
                    current_sh_idx != e.last_shard_index() && current_sh_idx != INVALID_SHARD_INDEX
                } {}
            });
        });

    }
}