/// Represents the type of a Component as an identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ComponentTypeId(u16);

impl From<u16> for ComponentTypeId {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl ComponentTypeId {
    pub const INVALID: ComponentTypeId = ComponentTypeId::from_u16(u16::MAX);

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0
    }

    #[inline(always)]
    pub const fn from_u16(v: u16) -> Self {
        Self(v)
    }

    #[inline(always)]
    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

impl ComponentTypeId {
    #[inline(always)]
    pub const fn to_ne_bytes(self) -> [u8; 2] {
        self.0.to_ne_bytes()
    }
}
