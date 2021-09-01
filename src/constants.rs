/// Used amount of bits in the entityhandle for storing the index.
/// Is defined to be 24 bits. Thus, yielding 2^24 - 1 = 16_777_215 different indices/entities.
pub const ENTITY_HANDLE_BITS: u8 = 24;

/// The maximum value that an entity can be.
pub const MAX_ENTITY_HANDLE_VALUE: u32 = 16_777_215;

/// Value indicating that an entity handle is invalid.
/// The null entity is the entity having the handle value being this value, and verison is max version value.
/// This corresponds to 2^24.
pub const INVALID_ENTITY_HANDLE_VALUE: u32 = 16_777_216;

/// Used amount of bits in an entityhandle for storing the version.
/// Is defined to be 8 bits. Thus, yielding 2^8 = 256 different versions.
pub const ENTITY_VERSION_BITS: u8 = 8;

/// The maximum value that an entity version can be.
pub const MAX_ENTITY_VERSION_VALUE: u8 = 255;

pub const MAX_COMPONENTS_PER_ENTITY: usize = 16;

pub const ENTITIES_PER_SHARD: usize = 3072;
