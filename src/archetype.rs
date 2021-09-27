use crate::archetype_descriptor::*;
use alloc::vec::*;

#[derive(Debug, Clone)]
pub(crate) struct Archetype {
    descriptor: ArchetypeDescriptor,
    shard_indices: Vec<u16>,
}

impl Archetype {
    pub(crate) fn new(descriptor: ArchetypeDescriptor, shard_indices: Vec<u16>) -> Self {
        Self {
            descriptor,
            shard_indices,
        }
    }

    /// Get a mutable reference to the archetype's shard indices.
    pub(crate) fn shard_indices_mut(&mut self) -> &mut Vec<u16> {
        &mut self.shard_indices
    }

    /// Get a reference to the archetype's descriptor.
    pub(crate) fn descriptor(&self) -> &ArchetypeDescriptor {
        &self.descriptor
    }
}
