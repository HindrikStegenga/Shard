use alloc::vec::Vec;

#[cfg(test)]
mod tests;

pub mod entry;
pub mod iterators;
pub mod registry;
pub mod entity;

pub use iterators::EntityIter;
pub use entity::Entity;