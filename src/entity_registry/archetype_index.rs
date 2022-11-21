#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeIndex {
    value: u16,
}

impl ArchetypeIndex {
    /// u16::MAX
    pub const INVALID_VALUE: u16 = u16::MAX;

    pub const fn new(value: u16) -> Option<ArchetypeIndex> {
        if value == Self::INVALID_VALUE {
            return None;
        }
        Some(Self { value })
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}
