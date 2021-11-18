use alloc::vec::Vec;

use crate::{
    Entity, ENTITIES_PER_SHARD, INVALID_ARCHETYPE_INDEX, INVALID_ENTITY_HANDLE_VALUE,
    MAX_COMPONENTS_PER_ENTITY, MAX_ENTITIES_PER_ARCHETYPE, MAX_ENTITY_HANDLE_VALUE,
};

/// Represents entity reference to the archetype + index + Version.
/// MEMORY_LAYOUTS:
/// Valid:      |version: u8|idx_in_arch: u24|arch_idx: u16|
/// Invalid:    |version: u8|next_fr_slt: u24|INV_ARCH: u16|
#[repr(packed)]
pub(crate) struct EntityEntry {
    values: Entity,
    arch_idx: u16,
}

impl EntityEntry {
    #[inline(always)]
    pub fn version(&self) -> u8 {
        self.values.version()
    }
    #[inline(always)]
    pub fn set_version(&mut self, version: u8) {
        self.values.set_version(version)
    }
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.arch_idx != INVALID_ARCHETYPE_INDEX
    }
    #[inline(always)]
    pub fn set_invalid(&mut self) {
        self.arch_idx = INVALID_ARCHETYPE_INDEX
    }

    #[inline(always)]
    pub fn set_archetype_index(&mut self, archetype_index: u16) {
        self.arch_idx = archetype_index;
    }
    #[inline(always)]
    pub fn archetype_index(&self) -> u16 {
        self.arch_idx
    }
    #[inline(always)]
    pub fn index_in_archetype(&self) -> u32 {
        self.values.index()
    }
    #[inline(always)]
    pub fn set_index_in_archetype(&mut self, index: u32) {
        unsafe { self.values.set_index(index) }
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

    /// Sets the archetype index. Panics in debug mode on invalid values!!
    #[inline(always)]
    pub(crate) fn set_archetype_index(&mut self, archetype_index: u16) {
        debug_assert!(archetype_index != INVALID_ARCHETYPE_INDEX);
        self.entry.set_archetype_index(archetype_index);
    }

    /// Sets the index in the archetype. Panics in debug mode on invalid values!!
    #[inline(always)]
    pub(crate) fn set_index_in_archetype(&mut self, index_in_archetype: u32) {
        debug_assert!(index_in_archetype < MAX_ENTITIES_PER_ARCHETYPE);
        self.entry.set_index_in_archetype(index_in_archetype);
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

        return if self.next_free_slot == INVALID_ENTITY_HANDLE_VALUE {
            // Linked list of free slots is empty, we need to allocate a new entity.
            self.entities.push(EntityEntry {
                values: unsafe { Entity::new(0, 0) },
                arch_idx: 0,
            });
            let idx = self.entities.len() - 1;
            Some(ValidEntityRef {
                entity: unsafe { Entity::new(idx as u32, 0) },
                entry: &mut self.entities[idx],
            })
        } else {
            let old_slot_index = self.next_free_slot;
            let entry = &mut self.entities[old_slot_index as usize];
            self.next_free_slot = unsafe { entry.index_in_archetype() };
            Some(ValidEntityRef {
                entity: unsafe { Entity::new(old_slot_index, entry.version()) },
                entry,
            })
        };
    }

    /// Registers a new entity into the registry.
    pub(crate) fn create_entity_with(
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
            self.entities.push(EntityEntry {
                values: unsafe { Entity::new(index_in_archetype, 0) },
                arch_idx: archetype_index,
            });
            Some(unsafe { Entity::new((self.entities.len() - 1) as u32, 0) })
        } else {
            let old_slot_index = self.next_free_slot;
            let mut entry = &mut self.entities[old_slot_index as usize];
            self.next_free_slot = unsafe { entry.index_in_archetype() };
            entry.set_index_in_archetype(index_in_archetype);
            entry.set_archetype_index(archetype_index);
            Some(unsafe { Entity::new(old_slot_index, entry.version()) })
        };
    }

    pub(crate) fn get_entity_entry(&self, entity: Entity) -> Option<&EntityEntry> {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return None;
        }
        let entry = &self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return None;
        }
        Some(entry)
    }

    pub(crate) fn get_entity_entry_mut(&mut self, entity: Entity) -> Option<&mut EntityEntry> {
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
    pub(crate) fn destroy_entity(&mut self, entity: Entity) -> bool {
        if entity.index() as usize >= self.entities.len() || entity == Entity::INVALID {
            return false;
        }
        let mut entry = &mut self.entities[entity.index() as usize];
        if entry.version() != entity.version() || !entry.is_valid() {
            return false;
        }
        entry.set_invalid();
        entry.set_version(entry.version().wrapping_add(1));
        entry.set_index_in_archetype(entity.index());
        self.next_free_slot = entity.index();
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;
    extern crate std;
    #[test]
    fn test_entity_registry() {
        let mut registry = EntityRegistry::default();

        assert!(!registry.destroy_entity(Entity::invalid()));

        let valid_entities = (0..MAX_ENTITY_HANDLE_VALUE)
            .into_iter()
            .filter_map(|_| registry.create_entity_with(1, 2))
            .collect::<Vec<_>>();
        valid_entities.iter().rev().for_each(|entity| {
            let entry = registry.get_entity_entry(*entity).unwrap();
            assert_eq!(entry.version(), 0);
            assert_eq!(entry.archetype_index(), 1);
            assert_eq!(entry.index_in_archetype(), 2);
            assert!(registry.destroy_entity(*entity));
        });
        valid_entities.iter().for_each(|entity| {
            assert_eq!(registry.destroy_entity(*entity), false);
            assert!(registry.get_entity_entry(*entity).is_none());
            assert!(registry.get_entity_entry_mut(*entity).is_none());
        });

        assert!(!registry.destroy_entity(Entity::invalid()));

        let valid_entities = (0..MAX_ENTITY_HANDLE_VALUE)
            .into_iter()
            .filter_map(|_| {
                let mut e = registry.create_entity()?;
                e.set_archetype_index(1);
                e.set_index_in_archetype(2);
                Some(e.entity())
            })
            .collect::<Vec<_>>();
        valid_entities.iter().rev().for_each(|entity| {
            let entry = registry.get_entity_entry(*entity).unwrap();
            assert_eq!(entry.version(), 0);
            assert_eq!(entry.archetype_index(), 1);
            assert_eq!(entry.index_in_archetype(), 2);
            assert!(registry.destroy_entity(*entity));
        });
        valid_entities.iter().for_each(|entity| {
            assert_eq!(registry.destroy_entity(*entity), false);
            assert!(registry.get_entity_entry(*entity).is_none());
            assert!(registry.get_entity_entry_mut(*entity).is_none());
        });
    }
}
