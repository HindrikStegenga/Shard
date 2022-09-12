use crate::constants::INVALID_ARCHETYPE_INDEX;

/// Represents entity reference to the archetype + index + Version.
/// MEMORY_LAYOUTS:
/// Valid:      |version: u8|idx_in_arch: u24|arch_idx: u16|
/// Invalid:    |version: u8|next_fr_slt: u24|INV_ARCH: u16|
///             |     0     | 1    2     3   | 4       5   |
#[repr(C, align(2))]
#[derive(Default, Clone, Debug)]
pub struct EntityEntry {
    values: [u8; 6],
}

impl EntityEntry {
    /// Returns the version of the entry.
    pub fn version(&self) -> u8 {
        self.values[0]
    }
    /// Sets the version of the entity.
    pub fn set_version(&mut self, version: u8) {
        self.values[0] = version;
    }
    /// Returns true if the entity entry is valid.
    pub fn is_valid(&self) -> bool {
        self.archetype_index() != INVALID_ARCHETYPE_INDEX
    }
    /// Sets the entry to invalid.
    pub fn set_invalid(&mut self) {
        self.set_archetype_index(INVALID_ARCHETYPE_INDEX);
    }
    /// Sets the archetype index.
    pub fn set_archetype_index(&mut self, archetype_index: u16) {
        let bytes = archetype_index.to_ne_bytes();
        self.values[4] = bytes[0];
        self.values[5] = bytes[1];
    }
    /// Returns the archetype index of the entry.
    pub fn archetype_index(&self) -> u16 {
        u16::from_ne_bytes([self.values[4], self.values[5]])
    }
    /// Returns the index in archetype of the entry.
    pub fn index_in_archetype(&self) -> u32 {
        let value = u32::from_ne_bytes([
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
        ]);
        (value & 0x00FFFFFF) >> 8
    }
    /// Sets the index in archetype for the entry.
    pub fn set_index_in_archetype(&mut self, index: u32) {
        let version_byte = self.values[0];
        let index = index << 8;
        let bytes = index.to_ne_bytes();

        self.values[0] = bytes[0];
        self.values[1] = bytes[1];
        self.values[2] = bytes[2];
        self.values[3] = bytes[3];
        self.values[0] = version_byte;
    }
}
