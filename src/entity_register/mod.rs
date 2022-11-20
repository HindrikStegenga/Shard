mod archetype_index;
mod entry;
mod index_in_archetype;

use crate::Entity;
use alloc::vec::*;
use entry::*;
use index_in_archetype::*;

use self::archetype_index::ArchetypeIndex;

pub struct ValidEntityRef<'registry> {
    entity: Entity,
    entry: &'registry mut EntityEntry,
}

impl<'registry> ValidEntityRef<'registry> {
    /// Returns the entity
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Sets the archetype index.
    pub fn set_archetype_index(&mut self, archetype_index: ArchetypeIndex) -> &mut Self {
        self.entry.set_archetype_index(archetype_index.value());
        self
    }

    /// Sets the index in the archetype.
    pub fn set_index_in_archetype(&mut self, index_in_archetype: IndexInArchetype) -> &mut Self {
        self.entry.set_index_in_archetype(index_in_archetype);
        self
    }
}

pub struct EntityRegister {
    entities: Vec<EntityEntry>,
    next_free_slot: u32,
}

impl Default for EntityRegister {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            next_free_slot: IndexInArchetype::INVALID_VALUE,
        }
    }
}

impl EntityRegister {
    pub const MAX_ENTITY_COUNT: usize = 100;

    pub fn can_register_new_entity(&self) -> bool {
        self.entities.len() < Self::MAX_ENTITY_COUNT
    }

    pub fn register_new_entity(&mut self, index_in_archetype: IndexInArchetype, archetype_index: ArchetypeIndex) -> Option<ValidEntityRef> {
        if self.entities.len() >= Self::MAX_ENTITY_COUNT {
            return None;
        }
        let mut valid_entity_ref = if self.next_free_slot == IndexInArchetype::INVALID_VALUE {
            // Linked list of free slots is empty, so we need to allocate a new entity.
            self.entities.push(EntityEntry::default());
            let idx = self.entities.len() - 1;
            ValidEntityRef {
                entity: unsafe { Entity::new_unchecked(idx as u32, 0) },
                entry: self.entities.last_mut().unwrap(),
            }
        } else {
            let slot = self.next_free_slot;
            let entry = &mut self.entities[slot as usize];
            self.next_free_slot = entry.index_in_archetype().value();
            ValidEntityRef {
                entity: unsafe { Entity::new_unchecked(slot as u32, entry.version()) },
                entry: self.entities.last_mut().unwrap(),
            }
        };
        valid_entity_ref.set_archetype_index(archetype_index);
        valid_entity_ref.set_index_in_archetype(index_in_archetype);
        Some(valid_entity_ref)
    }

    pub fn deregister_entity(&mut self, entity: Entity) -> bool {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return false;
        }
        let entry = &mut self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return false;
        }
        unsafe { entry.invalidate(self.next_free_slot) };
        self.next_free_slot = entity.index();
        entry.set_version(entry.version().wrapping_add(1));
        true
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_linked_list(register: &EntityRegister) {
        if register.next_free_slot == IndexInArchetype::INVALID_VALUE {
            // None of the slots may contain a marker that it's an invalid slot.
            for elem in &register.entities {
                assert!(elem.is_valid());
            }
        } else {
            let mut value = register.next_free_slot;
            while value != IndexInArchetype::INVALID_VALUE {
                assert!(value as usize <= register.entities.len());
                let slot = &register.entities[value as usize];
                assert!(!slot.is_valid());
                value = slot.index_in_archetype().value();
            }
        }
        
    }

    #[test]
    fn test_entity_register() {
        let mut register = EntityRegister::default();
        let entity = register
            .register_new_entity(IndexInArchetype::new(1).unwrap(), ArchetypeIndex::new(1).unwrap())
            .unwrap().entity();
        
        assert!(register.get_entity_entry(entity).is_some());
        verify_linked_list(&register);



    }
}
