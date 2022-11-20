#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexInArchetype {
    value: u32,
}

impl IndexInArchetype {
    // Equivalent to 2^24 - 1, as the stored value is [0 - 2^24) excluding 2^24 itself.
    pub const INVALID_VALUE: u32 = 16777215;

    /// Constructs a new IndexInArchetype using `value`.
    /// Returns `None` in case `value` >= Self::INVALID_VALUE.
    pub const fn new(value: u32) -> Option<IndexInArchetype> {
        if value >= Self::INVALID_VALUE {
            return None;
        }
        Some(Self { value })
    }
    /// Constructs a new IndexInArchetype using `value`.
    /// If value >= `Self::INVALID_VALUE` behaviour is undefined.
    pub const unsafe fn new_unchecked(value: u32) -> IndexInArchetype {
        Self { value }
    }

    /// Constructs a new IndexInArchetype from raw bytes.
    pub const fn from_bytes(bytes: [u8; 3]) -> IndexInArchetype {
        let mut value = 0;
        value += (bytes[0] as u32) << 16;
        value += (bytes[1] as u32) << 8;
        value += bytes[2] as u32;
        Self { value }
    }

    /// Returns the triple byte representation of the value.
    /// This is a shifted representation storing only the 3 most significant bytes.
    pub const fn to_bytes(&self) -> [u8; 3] {
        [
            ((self.value & 0x00FF0000) >> 16) as u8,
            ((self.value & 0x0000FF00) >> 8) as u8,
            (self.value & 0x000000FF) as u8,
        ]
    }

    /// Returns the raw numeric value of the index in archetype.
    pub const fn value(&self) -> u32 {
        self.value
    }
}

impl Default for IndexInArchetype {
    fn default() -> Self {
        Self { value: 0 }
    }
}
