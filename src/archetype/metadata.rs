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
    pub(crate) const fn new(entity: Entity) -> Self {
        Self {
            value: unsafe { Entity::new_unchecked(entity.index(), 0) },
        }
    }

    /// Get a reference to the entity metadata's value.
    pub(crate) const fn entity(&self) -> Entity {
        unsafe { Entity::new_unchecked(self.value.index(), 0) }
    }

    /// Returns the metadata byte associated with the entity.
    #[allow(dead_code)]
    pub(crate) const fn meta_byte(&self) -> u8 {
        self.value.version()
    }
}
