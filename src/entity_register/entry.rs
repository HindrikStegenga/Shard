use super::*;
use crate::INVALID_ARCHETYPE_INDEX;

/// Represents entity reference to the [ version | index_in_archetype | archetype_index | length | custom_bits ].
/// MEMORY_LAYOUTS:
/// Valid:      |version: u8|idx_in_arch: u24|arch_idx: u16| archetype_length: u8 | custom_bits: u8 ]
/// Invalid:    |version: u8|next_fr_slt: u24|INV_ARCH: u16| undefined: u8        | undefined: u8   ]
///             |     0     | 1    2     3   | 4       5   |        6             |        7        ]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct EntityEntry {
    values: [u8; 8],
}

impl Default for EntityEntry {
    fn default() -> Self {
        Self {
            values: [
                0, // version               byte 1
                0, // index_in_archetype    byte 1
                0, // index_in_archetype    byte 2
                0, // index_in_archetype    byte 3
                0, // archetype_index       byte 1
                0, // archetype_index       byte 2
                0, // archetype_length      byte 1
                0, // custom_bits           byte 1
            ],
        }
    }
}

impl EntityEntry {
    pub const fn version(&self) -> u8 {
        return self.values[0];
    }
    pub fn set_version(&mut self, version: u8) {
        self.values[0] = version;
    }
    pub const fn index_in_archetype(&self) -> IndexInArchetype {
        IndexInArchetype::from_bytes([self.values[1], self.values[2], self.values[3]])
    }
    pub fn set_index_in_archetype(&mut self, index_in_archetype: IndexInArchetype) {
        self.values[1] = index_in_archetype.to_bytes()[0];
        self.values[2] = index_in_archetype.to_bytes()[1];
        self.values[3] = index_in_archetype.to_bytes()[2];
    }
    pub const fn archetype_index(&self) -> u16 {
        u16::from_ne_bytes([self.values[4], self.values[5]])
    }
    pub fn set_archetype_index(&mut self, archetype_index: u16) {
        let bytes = archetype_index.to_ne_bytes();
        self.values[4] = bytes[0];
        self.values[5] = bytes[1];
    }
    pub const fn archetype_length(&self) -> u8 {
        self.values[6]
    }
    pub fn set_archetype_length(&mut self, archetype_length: u8) {
        self.values[6] = archetype_length;
    }
    pub const fn user_defined_metadata(&self) -> u8 {
        self.values[7]
    }
    pub fn set_user_defined_metadata(&mut self, metadata: u8) {
        self.values[7] = metadata;
    }
    /// Checks if this entry points to a valid entity.
    pub const fn is_valid(&self) -> bool {
        return self.archetype_index() != INVALID_ARCHETYPE_INDEX;
    }
    /// Sets the archetype index to invalid, indicating this entry does not point to a existing entity.
    /// # Safety
    /// - next_free_slot must at most be 24 bits.
    pub unsafe fn invalidate(&mut self, next_free_slot: u32) {
        self.set_index_in_archetype(IndexInArchetype::new_unchecked(next_free_slot));
        self.set_archetype_index(INVALID_ARCHETYPE_INDEX);
    }
}

#[cfg(test)]
mod tests {
    use super::{EntityEntry, IndexInArchetype};

    const TEST_VALUES_U8: [u8; 10] = [0, 3, 7, 12, 89, u8::MAX - 1, u8::MAX, 72, 134, 1];
    const TEST_VALUES_U16: [u16; 10] = [0, 1, 3, 7, 234, 29304, 13032, u16::MAX, u16::MAX - 1, 1];
    const TEST_VALUES_U32: [u32; 10] = [
        0,
        1,
        3,
        7,
        234,
        2429304,
        13032,
        IndexInArchetype::INVALID_VALUE - 1,
        2304820,
        1,
    ];

    /// Checks if bytes that are not in the mutation_allowed list are modified.
    /// Returns true if they're not mutated.
    fn check_bytes(
        before_bytes: &[u8; 8],
        after_bytes: &[u8; 8],
        mutation_allowed: &[usize],
    ) -> bool {
        for i in 0..before_bytes.len() {
            if mutation_allowed.contains(&i) {
                continue;
            }
            if before_bytes[i] != after_bytes[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_entity_register_enty() {
        let mut entry = EntityEntry::default();

        assert_eq!(entry.version(), 0);
        assert_eq!(entry.user_defined_metadata(), 0);
        assert_eq!(entry.archetype_length(), 0);
        assert_eq!(entry.archetype_index(), 0);
        assert_eq!(entry.index_in_archetype().value(), 0);

        // Test IndexInArchetype
        for value in TEST_VALUES_U32 {
            let idx = IndexInArchetype::new(value).unwrap();
            assert_eq!(idx.value(), value);
        }

        // Test version field
        for value in TEST_VALUES_U8 {
            let before_bytes = entry.values;
            entry.set_version(value);
            assert_eq!(entry.version(), value);
            assert!(check_bytes(&before_bytes, &entry.values, &[0]));
        }
        // Test user defined metadata field
        for value in TEST_VALUES_U8 {
            let before_bytes = entry.values;
            entry.set_user_defined_metadata(value);
            assert_eq!(entry.user_defined_metadata(), value);
            assert!(check_bytes(&before_bytes, &entry.values, &[7]));
        }
        // Test archetype_length field
        for value in TEST_VALUES_U8 {
            let before_bytes = entry.values;
            entry.set_archetype_length(value);
            assert_eq!(entry.archetype_length(), value);
            assert!(check_bytes(&before_bytes, &entry.values, &[6]));
        }
        // Test archetype index field.
        for value in TEST_VALUES_U16 {
            let before_bytes = entry.values;
            entry.set_archetype_index(value);
            assert_eq!(entry.archetype_index(), value);
            assert!(check_bytes(&before_bytes, &entry.values, &[4, 5]));
        }
        // Test index in archetype field.
        for value in TEST_VALUES_U32 {
            let before_bytes = entry.values;
            entry.set_index_in_archetype(unsafe { IndexInArchetype::new_unchecked(value) });
            assert_eq!(entry.index_in_archetype().value(), value);
            assert!(check_bytes(&before_bytes, &entry.values, &[1, 2, 3]));
        }
    }
}
