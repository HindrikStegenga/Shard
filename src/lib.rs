#![no_std]

extern crate alloc;

pub mod archetype;
pub mod archetype_registry;
pub mod constants;
pub mod entity_registry;
pub mod fnv1a;
pub mod registry;
pub mod descriptors;

#[cfg(test)]
mod test_components;

pub use registry::Registry;
pub use archetype::Archetype;
pub use entity_registry::Entity;
pub use descriptors::*;
pub use constants::*;

#[cfg(feature = "derive")]
pub use shard_ecs_derive::*;