use alloc::vec::{IntoIter, Vec};

use crate::{
    Entity, INVALID_ARCHETYPE_INDEX, INVALID_ENTITY_HANDLE_VALUE, MAX_ENTITIES_PER_ARCHETYPE,
    MAX_ENTITY_HANDLE_VALUE,
};

#[cfg(test)]
mod tests;

mod iterators;

use iterators::*;

/// Represents entity reference to the archetype + index + Version.
/// MEMORY_LAYOUTS:
/// Valid:      |version: u8|idx_in_arch: u24|arch_idx: u16|
/// Invalid:    |version: u8|next_fr_slt: u24|INV_ARCH: u16|
#[repr(C, align(2))]
#[derive(Clone, Debug)]
pub(crate) struct EntityEntry {
    values: [u8; 6],
}

impl EntityEntry {
    #[inline(always)]
    pub fn version(&self) -> u8 {
        self.values[0]
    }
    #[inline(always)]
    pub fn set_version(&mut self, version: u8) {
        self.values[0] = version;
    }
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        // Guaranteed to be properly aligned.
        unsafe {
            *(self.values.as_ptr().offset(4) as *const u8 as *const u16) != INVALID_ARCHETYPE_INDEX
        }
    }
    #[inline(always)]
    pub fn set_invalid(&mut self) {
        // Guaranteed to be properly aligned.
        unsafe {
            *(self.values.as_ptr().offset(4) as *mut u8 as *mut u16) = INVALID_ARCHETYPE_INDEX
        }
    }

    #[inline(always)]
    pub fn set_archetype_index(&mut self, archetype_index: u16) {
        // Guaranteed to be properly aligned.
        unsafe { *(self.values.as_ptr().offset(4) as *mut u8 as *mut u16) = archetype_index }
    }
    #[inline(always)]
    pub fn archetype_index(&self) -> u16 {
        unsafe { *(self.values.as_ptr().offset(4) as *const u16) }
    }
    #[inline(always)]
    pub fn index_in_archetype(&self) -> u32 {
        unsafe { ((*(self.values.as_ptr() as *const u32)) & 0x00FFFFFF) >> 8 }
    }
    #[inline(always)]
    pub fn set_index_in_archetype(&mut self, index: u32) {
        let v = self.values[0];
        let index = index << 8;
        unsafe {
            (*(self.values.as_ptr() as *mut u32)) = index;
        }
        self.values[0] = v;
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
    #[allow(dead_code)]
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
            self.entities.push(EntityEntry { values: [0; 6] });
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
            let mut entry = EntityEntry { values: [0; 6] };
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
}

impl<'a> IntoIterator for &'a EntityRegistry {
    type Item = Entity;
    type IntoIter = EntityIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EntityIter::new(&self.entities)
    }
}
