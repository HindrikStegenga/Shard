use crate::Entity;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EntityMetadata {
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
    /// Get a reference to the entity metadata's value.
    pub(crate) fn entity(&self) -> Entity {
        unsafe { Entity::new(self.value.index(), 0) }
    }
}
