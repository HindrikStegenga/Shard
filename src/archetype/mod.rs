use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::*;
use alloc::alloc::{alloc, dealloc, Layout};
use core::mem::{align_of, size_of};

mod data_access;
mod metadata;
#[cfg(test)]
mod tests;

pub use metadata::*;

use metadata::*;

#[derive(Debug)]
pub(crate) struct Archetype {
    descriptor: ArchetypeDescriptor,
    pointers: [*mut u8; MAX_COMPONENTS_PER_ENTITY],
    entity_metadata: *mut EntityMetadata,
    entity_count: u32,
    capacity: u32,
}

impl Archetype {
    /// Creates an archetype without memory allocated for it.
    /// Panics if the provided archetype descriptor is invalid.
    pub fn new(archetype_descriptor: &ArchetypeDescriptor) -> Archetype {
        assert!(archetype_descriptor.is_valid());
        Self {
            descriptor: archetype_descriptor.clone(),
            pointers: [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY],
            entity_metadata: core::ptr::null_mut(),
            entity_count: 0,
            capacity: 0,
        }
    }

    /// Allocates an archetype with a given capacity for storing data into it.
    /// Panics if the provided archetype descriptor is invalid.
    /// Does not allocate if [`capacity`] exceeds [`MAX_ENTITIES_PER_ARCHETYPE`].
    /// Does not allocate if [`capacity`] is 0.
    /// Panics in case of allocation failures.
    pub fn with_capacity(archetype_descriptor: &ArchetypeDescriptor, capacity: u32) -> Archetype {
        assert!(archetype_descriptor.is_valid());
        let mut archetype = Self {
            descriptor: archetype_descriptor.clone(),
            pointers: [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY],
            entity_metadata: core::ptr::null_mut(),
            entity_count: 0,
            capacity: 0,
        };
        // Allocate
        if capacity > 0 && capacity <= MAX_ENTITIES_PER_ARCHETYPE {
            unsafe {
                let layout = Layout::from_size_align_unchecked(
                    size_of::<EntityMetadata>(),
                    align_of::<EntityMetadata>(),
                );
                archetype.entity_metadata = alloc(layout) as *mut EntityMetadata;
                assert_ne!(archetype.entity_metadata, core::ptr::null_mut());

                for (index, component) in archetype.descriptor.components().iter().enumerate() {
                    let layout = Layout::from_size_align_unchecked(
                        component.size as usize,
                        component.align as usize,
                    );
                    archetype.pointers[index] = alloc(layout);
                    assert_ne!(archetype.pointers[index], core::ptr::null_mut());
                }
                archetype.capacity = capacity;
            }
        }
        archetype
    }

    pub fn descriptor(&self) -> &ArchetypeDescriptor {
        &self.descriptor
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        // Archetype is empty if there is no capacity.
        if self.capacity == 0 {
            return;
        }
        unsafe {
            self.drop_entities();
            self.dealloc();
        }
    }
}
