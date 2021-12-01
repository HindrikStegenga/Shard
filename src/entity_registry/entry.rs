use crate::constants::INVALID_ARCHETYPE_INDEX;

/// Represents entity reference to the archetype + index + Version.
/// MEMORY_LAYOUTS:
/// Valid:      |version: u8|idx_in_arch: u24|arch_idx: u16|
/// Invalid:    |version: u8|next_fr_slt: u24|INV_ARCH: u16|
#[repr(C, align(2))]
#[derive(Clone, Debug)]
pub(crate) struct EntityEntry {
    values: [u8; 6],
}

impl Default for EntityEntry {
    fn default() -> Self {
        Self { values: [0; 6] }
    }
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
        // Guaranteed to be properly aligned.
        unsafe {
            *(self.values.as_ptr().offset(4) as *const u8 as *const u16) != INVALID_ARCHETYPE_INDEX
        }
    }
    /// Sets the entry to invalid.
    pub fn set_invalid(&mut self) {
        // Guaranteed to be properly aligned.
        unsafe {
            *(self.values.as_ptr().offset(4) as *mut u8 as *mut u16) = INVALID_ARCHETYPE_INDEX
        }
    }
    /// Sets the archetype index.
    pub fn set_archetype_index(&mut self, archetype_index: u16) {
        // Guaranteed to be properly aligned.
        unsafe { *(self.values.as_ptr().offset(4) as *mut u8 as *mut u16) = archetype_index }
    }
    /// Returns the archetype index of the entry.
    pub fn archetype_index(&self) -> u16 {
        unsafe { *(self.values.as_ptr().offset(4) as *const u16) }
    }
    /// Returns the index in archetype of the entry.
    pub fn index_in_archetype(&self) -> u32 {
        unsafe { ((*(self.values.as_ptr() as *const u32)) & 0x00FFFFFF) >> 8 }
    }
    /// Sets the index in archetype for the entry.
    pub fn set_index_in_archetype(&mut self, index: u32) {
        let v = self.values[0];
        let index = index << 8;
        unsafe {
            (*(self.values.as_ptr() as *mut u32)) = index;
        }
        self.values[0] = v;
    }
}
