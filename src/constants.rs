/// Used amount of bits in the entity handle for storing the index.
/// Is defined to be 24 bits. Thus, yielding 2^24 - 1 = 16_777_215 different indices/entities.
pub const ENTITY_HANDLE_BITS: u8 = 24;

/// The maximum value that an entity can be.
pub const MAX_ENTITY_HANDLE_VALUE: u32 = 16_777_215;

/// Value indicating that an entity handle is invalid.
/// The null entity is the entity having the handle value being this value, and version is max version value.
/// This corresponds to 2^24.
pub const INVALID_ENTITY_HANDLE_VALUE: u32 = 16_777_216;

/// Used amount of bits in an entity for storing the version.
/// Is defined to be 8 bits. Thus, yielding 2^8 = 256 different versions.
pub const ENTITY_VERSION_BITS: u8 = 8;

/// The maximum value that an entity version can be.
pub const MAX_ENTITY_VERSION_VALUE: u8 = 255;

/// The maximum number of components an entity is allowed to have.
pub const MAX_COMPONENTS_PER_ENTITY: usize = 14;

/// The maximum amount of unique archetypes that a registry can contain.
pub const MAX_ARCHETYPE_COUNT: usize = (u16::MAX - 1) as usize;

/// Valid archetype indices can never have this value.
pub const INVALID_ARCHETYPE_INDEX: u16 = u16::MAX;

/// Maximum amount of entities that can be stored for a given archetype.
pub const MAX_ENTITIES_PER_ARCHETYPE: u32 = MAX_ENTITY_HANDLE_VALUE;

/// The default amount of elements for which space is reserved if an entity is pushed into an empty archetype.
pub const DEFAULT_ARCHETYPE_ALLOCATION_SIZE: usize = 128;
