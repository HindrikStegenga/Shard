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
pub use entity_registry::Entity;
pub use descriptors::*;
pub use constants::{MAX_COMPONENTS_PER_ENTITY, MAX_ENTITIES_PER_ARCHETYPE};