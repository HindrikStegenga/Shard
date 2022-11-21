mod entity;
mod archetype_index;
mod entry;
mod iterator;
mod index_in_archetype;

pub use entity::*;
pub use entry::*;
pub use iterator::*;
pub use index_in_archetype::*;
pub use archetype_index::*;

use alloc::vec::*;

pub struct EntityRegistry {
    entities: Vec<EntityEntry>,
    next_free_slot: u32,
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            next_free_slot: IndexInArchetype::INVALID_VALUE,
        }
    }
}

impl EntityRegistry {
    pub const MAX_ENTITY_COUNT: usize = crate::MAX_ENTITY_HANDLE_VALUE as usize;

    pub fn can_register_new_entity(&self) -> bool {
        self.entities.len() < Self::MAX_ENTITY_COUNT
    }

    pub fn create_entity(&mut self, index_in_archetype: IndexInArchetype, archetype_index: ArchetypeIndex) -> Option<Entity> {
        if self.entities.len() >= Self::MAX_ENTITY_COUNT {
            return None;
        }
        let (entity, entry) = if self.next_free_slot == IndexInArchetype::INVALID_VALUE {
            // Linked list of free slots is empty, so we need to allocate a new entity.
            self.entities.push(EntityEntry::default());
            let idx = self.entities.len() - 1;
            (
                unsafe { Entity::new_unchecked(idx as u32, 0) },
                self.entities.last_mut().unwrap(),
            )
        } else {
            let slot = self.next_free_slot;
            let entry = &mut self.entities[slot as usize];
            self.next_free_slot = entry.index_in_archetype().value();
            (
                unsafe { Entity::new_unchecked(slot as u32, entry.version()) },
                &mut self.entities[slot as usize]
            )
        };
        entry.set_archetype_index(archetype_index.value());
        entry.set_index_in_archetype(index_in_archetype);
        Some(entity)
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
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

    pub fn entity_entry(&self, entity: Entity) -> Option<&EntityEntry> {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return None;
        }
        let entry = &self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return None;
        }
        Some(entry)
    }

    pub fn entity_entry_mut(&mut self, entity: Entity) -> Option<&mut EntityEntry> {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return None;
        }
        let entry = &mut self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return None;
        }
        Some(entry)
    }
    
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        EntityIter::new(&self.entities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_linked_list(register: &EntityRegistry) {
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
        let mut register = EntityRegistry::default();
        let index_in_archetype = IndexInArchetype::new(1).unwrap();
        let archetype_index = ArchetypeIndex::new(1).unwrap();

        let entity = register
            .create_entity(index_in_archetype, archetype_index)
            .unwrap();
        
        assert!(register.entity_entry(entity).is_some());
        verify_linked_list(&register);
        assert_eq!(register.next_free_slot, IndexInArchetype::INVALID_VALUE);
        assert_eq!(register.entities.len(), 1);

        assert!(register.destroy_entity(entity));
        assert_eq!(register.next_free_slot, entity.index());
        assert!(!register.entities[entity.index() as usize].is_valid());
        assert_eq!(register.entities[entity.index() as usize].version(), 1);
        assert_eq!(register.entities[entity.index() as usize].archetype_index(), ArchetypeIndex::INVALID_VALUE);
        assert!(register.entity_entry(entity).is_none());
        assert!(register.entity_entry_mut(entity).is_none());

        let entity = register.create_entity(index_in_archetype, archetype_index).unwrap();
        let entry = register.entity_entry(entity).unwrap();
        assert_eq!(entry.archetype_index(), archetype_index.value());
        assert_eq!(entry.index_in_archetype(), index_in_archetype);
        assert_eq!(entity.index(), 0);

        verify_linked_list(&register);
        assert_eq!(register.next_free_slot, IndexInArchetype::INVALID_VALUE);
        assert_eq!(register.entities.len(), 1);

    }

    const ENTITY_COUNT: u32 = 1024;
    #[test]
    fn test_many_entities() {
        let mut registry = EntityRegistry::default();
        // Check destroying invalid entity.
        assert!(!registry.destroy_entity(Entity::invalid()));
        let index_in_archetype = IndexInArchetype::new(2).unwrap();
        let archetype_index = ArchetypeIndex::new(1).unwrap();

        let entities = (0..ENTITY_COUNT)
            .into_iter()
            .filter_map(|_| registry.create_entity(index_in_archetype, archetype_index))
            .collect::<Vec<_>>();
        entities.iter().rev().cloned().for_each(|e| {
            let entry = registry.entity_entry(e).unwrap();
            assert_eq!(entry.version(), 0);
            assert_eq!(entry.archetype_index(), 1);
            assert_eq!(entry.index_in_archetype().value(), 2);
            assert!(registry.destroy_entity(e));
        });
        entities.iter().cloned().for_each(|e| {
            assert_eq!(registry.destroy_entity(e), false);
            assert!(registry.entity_entry(e).is_none());
            assert!(registry.entity_entry_mut(e).is_none());
        });

        assert!(!registry.destroy_entity(Entity::invalid()));

        let entities = (0..ENTITY_COUNT)
            .into_iter()
            .filter_map(|_| registry.create_entity(index_in_archetype, archetype_index))
            .collect::<Vec<_>>();
        entities.iter().rev().cloned().for_each(|entity| {
            let entry = registry.entity_entry(entity).unwrap();
            assert_ne!(entry.version(), 0);
            assert_eq!(entry.archetype_index(), 1);
            assert_eq!(entry.index_in_archetype().value(), 2);
            assert!(registry.destroy_entity(entity));
        });
        entities.iter().for_each(|entity| {
            assert!(!registry.destroy_entity(*entity));
            assert!(registry.entity_entry(*entity).is_none());
            assert!(registry.entity_entry_mut(*entity).is_none());
        });

        for entity in registry.iter() {
            let _ = entities.contains(&entity);
        }
    }
}
