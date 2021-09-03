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
    /// Registers a new entity into the registry.
    pub(crate) fn create_entity(
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
