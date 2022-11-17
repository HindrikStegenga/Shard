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
    pub fn set_archetype_index(&mut self, archetype_index: ArchetypeIndex) {
        self.entry.set_archetype_index(archetype_index.value());
    }

    /// Sets the index in the archetype.
    pub fn set_index_in_archetype(&mut self, index_in_archetype: IndexInArchetype) {
        self.entry.set_index_in_archetype(index_in_archetype);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_entity_register() {}
}
