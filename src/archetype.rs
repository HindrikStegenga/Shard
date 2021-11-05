use crate::archetype_descriptor::*;
use alloc::vec::*;

#[derive(Debug, Clone)]
pub(crate) struct Archetype {
    descriptor: ArchetypeDescriptor,
    first_shard_index: u16,
    last_shard_index: u16,
}

impl Archetype {
    pub(crate) fn new(descriptor: ArchetypeDescriptor, first_shard_index: u16) -> Self {
        Self {
            descriptor,
            first_shard_index,
            last_shard_index: first_shard_index,
        }
    }

    /// Get a reference to the archetype's descriptor.
    pub(crate) fn descriptor(&self) -> &ArchetypeDescriptor {
        &self.descriptor
    }
}
