#![no_std]

extern crate alloc;

pub mod archetype;
pub mod archetype_registry;
pub mod constants;
pub mod descriptors;
pub mod entity_registry;
pub mod fnv1a;
pub mod registry;

pub use archetype::Archetype;
pub use constants::*;
pub use descriptors::*;
pub use entity_registry::*;
pub use registry::Registry;

#[cfg(test)]
mod test_components;

#[cfg(feature = "derive")]
pub use shard_ecs_derive::*;