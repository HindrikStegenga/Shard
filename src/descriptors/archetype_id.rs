/// Represents the unique subset of components as a comparable identifier.
/// See [`ArchetypeDescriptor::compute_archetype_id`] for computing an instance of it.
/// The invalid archetype id is defined to be 0.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeId(u32);

impl From<u32> for ArchetypeId {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl ArchetypeId {
    /// The invalid archetype id, defined to be zero.
    /// Any archetype with this id must not be used as valid archetype.
    pub const INVALID: ArchetypeId = ArchetypeId::from_u32(u32::MAX);
    /// Returns true if the archetype id is valid.
    pub const fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0
    }
    /// Construct an archetype id from a u32.
    pub const fn from_u32(v: u32) -> Self {
        ArchetypeId(v)
    }
    /// Construct a u32 from an archetype id.
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

impl ArchetypeId {
    /// Copies the value into an array of ne_bytes. (See [`u32::to_ne_bytes`]).
    pub const fn to_ne_bytes(self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }
}
