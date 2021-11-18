use crate::Entity;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct EntityMetadata {
    value: Entity,
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self {
            value: Entity::INVALID,
        }
    }
}

impl EntityMetadata {
    /// Creates a new metadata pointing to the given entity.
    /// The 8 metadata bits are set to 0.
    pub(crate) fn new(entity: Entity) -> Self {
        Self {
            value: unsafe { Entity::new(entity.index(), 0) },
        }
    }

    /// Get a reference to the entity metadata's value.
    pub(crate) fn entity(&self) -> Entity {
        unsafe { Entity::new(self.value.index(), 0) }
    }
}
