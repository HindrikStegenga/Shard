use alloc::vec::Vec;

use crate::{
    Entity, INVALID_ARCHETYPE_INDEX, INVALID_ENTITY_HANDLE_VALUE, MAX_ENTITIES_PER_ARCHETYPE,
    MAX_ENTITY_HANDLE_VALUE,
};

#[cfg(test)]
mod tests;

mod entry;
mod iterators;
mod registry;

use entry::*;
use iterators::*;

pub(crate) use registry::*;
