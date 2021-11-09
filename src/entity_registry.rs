use alloc::vec::Vec;

use crate::{
    Entity, ENTITIES_PER_SHARD, INVALID_ENTITY_HANDLE_VALUE, MAX_COMPONENTS_PER_ENTITY,
    MAX_ENTITY_HANDLE_VALUE,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ValidEntityEntry {
    shard_index: u16,
    index_in_shard: u16,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct InvalidEntityEntry {
    next_free_slot: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union EntityEntryState {
    valid: ValidEntityEntry,
    invalid: InvalidEntityEntry,
}

#[repr(packed)]
pub(crate) struct EntityEntry {
    version: u8,
    archetype_length: u8,
    state: EntityEntryState,
}
impl EntityEntry {
    #[inline(always)]
    pub(crate) const fn is_valid(&self) -> bool {
        self.archetype_length != u8::MAX
    }

    #[inline(always)]
    pub(crate) fn set_shard_index(&mut self, shard_index: u16) {
        debug_assert!(self.is_valid());
        self.state.valid.shard_index = shard_index;
    }

    #[inline(always)]
    pub(crate) fn set_index_in_shard(&mut self, index_in_shard: u16) {
        debug_assert!(self.is_valid());
        self.state.valid.index_in_shard = index_in_shard;
    }

    #[inline(always)]
    pub(crate) fn set_archetype_length(&mut self, length: u8) {
        debug_assert!(self.is_valid());
        debug_assert!(length as usize <= MAX_COMPONENTS_PER_ENTITY);
        self.archetype_length = length;
    }

    #[inline(always)]
    pub(crate) fn index_in_shard(&self) -> u16 {
        debug_assert!(self.is_valid());
        unsafe { self.state.valid.index_in_shard }
    }

    #[inline(always)]
    pub(crate) fn shard_index(&self) -> u16 {
        debug_assert!(self.is_valid());
        unsafe { self.state.valid.shard_index }
    }

    #[inline(always)]
    pub(crate) fn archetype_length(&self) -> u8 {
        self.archetype_length
    }

    /// Get the entity entry's version.
    #[inline(always)]
    pub(crate) fn version(&self) -> u8 {
        self.version
    }
}

pub(crate) struct EntityRegistry {
    entities: Vec<EntityEntry>,
    next_free_slot: u32,
}

pub(crate) struct ValidEntityRef<'registry> {
    entity: Entity,
    entry: &'registry mut EntityEntry,
}

impl<'registry> ValidEntityRef<'registry> {
    // Returns the entity
    pub(crate) fn entity(&self) -> Entity {
        self.entity
    }

    /// Sets the shard index. Panics in debug mode on invalid values!!
    pub(crate) fn set_shard_index(&mut self, shard_index: u16) {
        self.entry.set_shard_index(shard_index)
    }

    /// Sets the index in shard. Panics in debug mode on invalid values!!
    pub(crate) fn set_index_in_shard(&mut self, index_in_shard: u16) {
        debug_assert!((index_in_shard as usize) < ENTITIES_PER_SHARD);
        self.entry.set_index_in_shard(index_in_shard)
    }

    /// Sets the archetype length. Panics in debug mode on invalid values!!
    pub(crate) fn set_archetype_length(&mut self, archetype_length: u8) {
        debug_assert!(archetype_length as usize <= MAX_COMPONENTS_PER_ENTITY);
        self.entry.archetype_length = archetype_length
    }
}

impl Default for EntityRegistry {
    #[inline(always)]
    fn default() -> Self {
        Self {
            entities: Vec::with_capacity(8192),
            next_free_slot: INVALID_ENTITY_HANDLE_VALUE,
        }
    }
}

impl EntityRegistry {
    /// Returns true if there is space left to store a new entity record.
    pub(crate) fn can_create_new_entity(&self) -> bool {
        !(self.entities.len() >= MAX_ENTITY_HANDLE_VALUE as usize)
    }

    /// Registers a new entity into the registry.
    pub(crate) fn create_entity(&mut self) -> Option<ValidEntityRef> {
        if self.entities.len() >= MAX_ENTITY_HANDLE_VALUE as usize {
            return None;
        }

        if self.next_free_slot == INVALID_ENTITY_HANDLE_VALUE {
            // Linked list of free slots is empty, we need to allocate a new entity.
            self.entities.push(EntityEntry {
                version: 0,
                archetype_length: 0,
                state: EntityEntryState {
                    valid: ValidEntityEntry {
                        shard_index: 0,
                        index_in_shard: 0,
                    },
                },
            });
            let idx = self.entities.len() - 1;
            return Some(ValidEntityRef {
                entity: unsafe { Entity::new(idx as u32, 0) },
                entry: &mut self.entities[idx],
            });
        } else {
            let old_slot_index = self.next_free_slot;
            let entry = &mut self.entities[old_slot_index as usize];
            self.next_free_slot = unsafe { entry.state.invalid.next_free_slot };
            return Some(ValidEntityRef {
                entity: unsafe { Entity::new(old_slot_index, entry.version) },
                entry: entry,
            });
        }
    }

    /// Registers a new entity into the registry.
    pub(crate) fn create_entity_with(
        &mut self,
        shard_idx: u16,
        index_in_shard: u16,
        archetype_length: u8,
    ) -> Option<Entity> {
        debug_assert!(archetype_length as usize <= MAX_COMPONENTS_PER_ENTITY);
        debug_assert!((index_in_shard as usize) < ENTITIES_PER_SHARD);

        if self.entities.len() >= MAX_ENTITY_HANDLE_VALUE as usize {
            return None;
        }

        if self.next_free_slot == INVALID_ENTITY_HANDLE_VALUE {
            // Linked list of free slots is empty, we need to allocate a new entity.
            self.entities.push(EntityEntry {
                version: 0,
                archetype_length: archetype_length,
                state: EntityEntryState {
                    valid: ValidEntityEntry {
                        shard_index: shard_idx,
                        index_in_shard,
                    },
                },
            });
            return Some(unsafe { Entity::new((self.entities.len() - 1) as u32, 0) });
        } else {
            let old_slot_index = self.next_free_slot;
            let mut entry = &mut self.entities[old_slot_index as usize];
            self.next_free_slot = unsafe { entry.state.invalid.next_free_slot };
            entry.state.valid.index_in_shard = index_in_shard;
            entry.state.valid.shard_index = shard_idx;
            entry.archetype_length = archetype_length;
            return Some(unsafe { Entity::new(old_slot_index, entry.version) });
        }
    }

    pub(crate) fn get_entity_entry(&self, entity: Entity) -> Option<&EntityEntry> {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return None;
        }
        let entry = &self.entities[entity.index() as usize];
        if entry.version != entity.version() || !entry.is_valid() {
            return None;
        }
        Some(entry)
    }

    pub(crate) fn get_entity_entry_mut(&mut self, entity: Entity) -> Option<&mut EntityEntry> {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return None;
        }
        let entry = &mut self.entities[entity.index() as usize];
        if entry.version != entity.version() || !entry.is_valid() {
            return None;
        }
        Some(entry)
    }

    /// Removes an entity from the registry.
    pub(crate) fn destroy_entity(&mut self, entity: Entity) -> bool {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return false;
        }
        let mut entry = &mut self.entities[entity.index() as usize];
        if entry.version != entity.version() || !entry.is_valid() {
            return false;
        }
        entry.archetype_length = u8::MAX;
        entry.version = entry.version.wrapping_add(1);
        entry.state.invalid.next_free_slot = self.next_free_slot;
        self.next_free_slot = entity.index();
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_registry() {
        let mut registry = EntityRegistry::default();

        assert!(!registry.destroy_entity(Entity::invalid()));

        let valid_entities = (0..MAX_ENTITY_HANDLE_VALUE)
            .into_iter()
            .filter_map(|_| registry.create_entity_with(1, 2, 3))
            .collect::<Vec<_>>();
        valid_entities.iter().rev().for_each(|entity| {
            let entry = registry.get_entity_entry(*entity).unwrap();
            assert_eq!(entry.version(), 0);
            assert_eq!(entry.shard_index(), 1);
            assert_eq!(entry.index_in_shard(), 2);
            assert_eq!(entry.archetype_length(), 3);
            assert!(registry.destroy_entity(*entity));
        });
        valid_entities.iter().for_each(|entity| {
            assert!(registry.destroy_entity(*entity) == false);
            assert!(registry.get_entity_entry(*entity).is_none());
            assert!(registry.get_entity_entry_mut(*entity).is_none());
        });

        assert!(!registry.destroy_entity(Entity::invalid()));

        let valid_entities = (0..MAX_ENTITY_HANDLE_VALUE)
            .into_iter()
            .filter_map(|_| {
                let mut e = registry.create_entity()?;
                e.set_shard_index(1);
                e.set_index_in_shard(2);
                e.set_archetype_length(3);
                Some(e.entity())
            })
            .collect::<Vec<_>>();
        valid_entities.iter().rev().for_each(|entity| {
            let entry = registry.get_entity_entry(*entity).unwrap();
            assert_eq!(entry.version(), 0);
            assert_eq!(entry.shard_index(), 1);
            assert_eq!(entry.index_in_shard(), 2);
            assert_eq!(entry.archetype_length(), 3);
            assert!(registry.destroy_entity(*entity));
        });
        valid_entities.iter().for_each(|entity| {
            assert!(registry.destroy_entity(*entity) == false);
            assert!(registry.get_entity_entry(*entity).is_none());
            assert!(registry.get_entity_entry_mut(*entity).is_none());
        });
    }
}
