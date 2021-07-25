/// Represents the unique subset of components as a comparable identifier.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeId(u32);

impl From<u32> for ArchetypeId {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl ArchetypeId {
    pub const INVALID: ArchetypeId = ArchetypeId::from_u32(u32::MAX);

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0
    }

    #[inline(always)]
    pub const fn from_u32(v: u32) -> Self {
        ArchetypeId(v)
    }

    #[inline(always)]
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

impl ArchetypeId {
    #[inline(always)]
    pub fn to_ne_bytes(self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }
}
