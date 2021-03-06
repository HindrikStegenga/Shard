use super::*;
use super::entry::*;
use crate::constants::*;

pub struct EntityRegistry {
    entities: Vec<EntityEntry>,
    next_free_slot: u32,
}

pub struct ValidEntityRef<'registry> {
    entity: Entity,
    entry: &'registry mut EntityEntry,
}

impl<'registry> ValidEntityRef<'registry> {
    /// Returns the entity
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Sets the archetype index. Panics in debug mode on invalid values!!
    pub fn set_archetype_index(&mut self, archetype_index: u16) {
        debug_assert!(archetype_index != INVALID_ARCHETYPE_INDEX);
        self.entry.set_archetype_index(archetype_index);
    }

    /// Sets the index in the archetype. Panics in debug mode on invalid values!!
    pub fn set_index_in_archetype(&mut self, index_in_archetype: u32) {
        debug_assert!(index_in_archetype < MAX_ENTITIES_PER_ARCHETYPE);
        self.entry.set_index_in_archetype(index_in_archetype);
    }
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self {
            entities: Vec::with_capacity(8192),
            next_free_slot: INVALID_ENTITY_HANDLE_VALUE,
        }
    }
}

impl EntityRegistry {
    /// Returns true if there is space left to store a new entity record.
    #[allow(dead_code)]
    pub fn can_create_new_entity(&self) -> bool {
        !(self.entities.len() >= MAX_ENTITY_HANDLE_VALUE as usize)
    }

    /// Registers a new entity into the registry.
    pub fn create_entity(&mut self) -> Option<ValidEntityRef> {
        if self.entities.len() >= MAX_ENTITY_HANDLE_VALUE as usize {
            return None;
        }

        return if self.next_free_slot == INVALID_ENTITY_HANDLE_VALUE {
            // Linked list of free slots is empty, we need to allocate a new entity.
            self.entities.push(EntityEntry::default());
            let idx = self.entities.len() - 1;
            Some(ValidEntityRef {
                entity: unsafe { Entity::new_unchecked(idx as u32, 0) },
                entry: &mut self.entities[idx],
            })
        } else {
            let old_slot_index = self.next_free_slot;
            let entry = &mut self.entities[old_slot_index as usize];
            self.next_free_slot = entry.index_in_archetype();
            Some(ValidEntityRef {
                entity: unsafe { Entity::new_unchecked(old_slot_index, entry.version()) },
                entry,
            })
        };
    }

    /// Registers a new entity into the registry.
    #[allow(dead_code)]
    pub fn create_entity_with(
        &mut self,
        archetype_index: u16,
        index_in_archetype: u32,
    ) -> Option<Entity> {
        debug_assert!((archetype_index as usize) != INVALID_ARCHETYPE_INDEX as usize);
        debug_assert!((index_in_archetype) < MAX_ENTITIES_PER_ARCHETYPE);

        if self.entities.len() >= MAX_ENTITY_HANDLE_VALUE as usize {
            return None;
        }

        return if self.next_free_slot == INVALID_ENTITY_HANDLE_VALUE {
            // Linked list of free slots is empty, we need to allocate a new entity.
            let mut entry = EntityEntry::default();
            entry.set_archetype_index(archetype_index);
            entry.set_index_in_archetype(index_in_archetype);
            self.entities.push(entry);
            Some(unsafe { Entity::new_unchecked((self.entities.len() - 1) as u32, 0) })
        } else {
            let old_slot_index = self.next_free_slot;
            let entry = &mut self.entities[old_slot_index as usize];
            self.next_free_slot = entry.index_in_archetype();
            entry.set_index_in_archetype(index_in_archetype);
            entry.set_archetype_index(archetype_index);
            Some(unsafe { Entity::new_unchecked(old_slot_index, entry.version()) })
        };
    }

    pub fn get_entity_entry(&self, entity: Entity) -> Option<&EntityEntry> {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return None;
        }
        let entry = &self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return None;
        }
        Some(entry)
    }

    pub fn get_entity_entry_mut(&mut self, entity: Entity) -> Option<&mut EntityEntry> {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return None;
        }
        let entry = &mut self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return None;
        }
        Some(entry)
    }

    /// Removes an entity from the registry.
    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return false;
        }
        let entry = &mut self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return false;
        }
        entry.set_invalid();
        entry.set_version(entry.version().wrapping_add(1));
        entry.set_index_in_archetype(entity.index());
        self.next_free_slot = entity.index();
        return true;
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Entity> + 'a {
        EntityIter::new(&self.entities)
    }
}
