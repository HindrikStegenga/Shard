use alloc::vec::Vec;

#[cfg(test)]
mod tests;

pub mod entity;
pub mod entry;
pub mod iterators;
pub mod registry;

pub use entity::Entity;
pub use iterators::EntityIter;
