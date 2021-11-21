use crate::*;

/// Represents an abstract Entity. Is internally a handle into the ECS to query it's associated components.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Entity {
    handle: u32,
}

impl Default for Entity {
    fn default() -> Self {
        Self::INVALID
    }
}

impl Entity {
    pub const INVALID: Entity =
        unsafe { Entity::new_unchecked(MAX_ENTITY_HANDLE_VALUE, MAX_ENTITY_VERSION_VALUE) };

    /// Constructs a new Entity from a raw u32.
    /// The higher/leftmost 24 bits as index, the lower/rightmost 8 bits are used as version.
    #[inline(always)]
    pub const unsafe fn from_raw(raw: u32) -> Entity {
        Self { handle: raw }
    }

    /// Returns if entity is valid, meaning it is NOT equal to Entity::INVALID.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.handle != Self::INVALID.handle
    }

    /// Returns the invalid entity.
    #[inline(always)]
    pub const fn invalid() -> Entity {
        Self::INVALID
    }

    /// Manually construct a new Entity. index MUST be lower than 2^24!
    /// Failing to uphold this invariant will corrupt the internal handle.
    /// # Safety
    /// - index MUST be lower than 2^24!
    #[inline(always)]
    pub const unsafe fn new_unchecked(index: u32, version: u8) -> Entity {
        Entity {
            handle: (index << ENTITY_VERSION_BITS) | version as u32,
        }
    }

    /// Returns the index part of the entity's handle.
    #[inline(always)]
    pub const fn index(&self) -> u32 {
        self.handle >> ENTITY_VERSION_BITS
    }

    /// Sets the index part of the entity's handle.
    /// index MUST be lower than 2^24!
    /// Failing to uphold this invariant will corrupt the internal handle.
    /// # Safety
    /// - index MUST be lower than 2^24!
    #[inline(always)]
    pub unsafe fn set_index(&mut self, index: u32) {
        debug_assert!(
            index < 2u32.pow(ENTITY_HANDLE_BITS as u32),
            "Entity index must be < 2^24!"
        );

        let version: u8 = (0xFFFFFF & self.handle) as u8;
        self.handle = (index << ENTITY_VERSION_BITS) | version as u32;
    }

    /// Returns the version part of the entity's handle.
    #[inline(always)]
    pub const fn version(&self) -> u8 {
        (0xFFFFFF & self.handle) as u8
    }

    /// Sets the version part of the entity's handle.
    #[inline(always)]
    pub fn set_version(&mut self, version: u8) {
        self.handle = (self.index() << ENTITY_VERSION_BITS) | version as u32;
    }

    /// Returns the raw entity handle.
    #[inline(always)]
    pub const fn raw(&self) -> u32 {
        self.handle
    }
}

#[test]
fn test_entity_handles() {
    let mut entity = unsafe { Entity::new_unchecked(8_000_000, 255) };
    assert_eq!(entity.index(), 8_000_000);
    assert_eq!(entity.version(), 255);

    entity.set_version(20);
    assert_eq!(entity.version(), 20);
    assert_eq!(entity.index(), 8_000_000);

    unsafe {
        entity.set_index(30);
    }
    assert_eq!(entity.index(), 30);
    assert_eq!(entity.version(), 20);

    assert_eq!(Entity::invalid().raw(), u32::MAX);
}
