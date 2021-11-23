mod sorted_archetype_key;
#[cfg(test)]
mod tests;

use alloc::vec::*;
use core::ops::{Index, IndexMut};
use sorted_archetype_key::*;

use crate::archetype::Archetype;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::constants::*;

const DEFAULT_VECTOR_CAPACITY: usize = 64;

#[derive(Debug)]
pub(crate) struct ArchetypeRegistry {
    sorted_mappings: [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
    archetypes: Vec<Archetype>,
}

impl Default for ArchetypeRegistry {
    fn default() -> Self {
        Self {
            sorted_mappings: [
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
                Vec::with_capacity(DEFAULT_VECTOR_CAPACITY),
            ],
            archetypes: Vec::with_capacity(128),
        }
    }
}

impl ArchetypeRegistry {
    #[allow(dead_code)]
    pub(crate) fn find_archetype(
        &self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<&Archetype> {
        let len = archetype_descriptor.len() as usize;
        if len > MAX_COMPONENTS_PER_ENTITY || !archetype_descriptor.is_valid() {
            return None;
        }
        return match self.sorted_mappings[len]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => Some(
                &self.archetypes[self.sorted_mappings[len][found_index].archetype_index as usize],
            ),
            Err(_) => None,
        };
    }

    #[allow(dead_code)]
    pub(crate) fn find_archetype_mut(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<&mut Archetype> {
        let len = archetype_descriptor.len() as usize;
        if len > MAX_COMPONENTS_PER_ENTITY || !archetype_descriptor.is_valid() {
            return None;
        }
        return match self.sorted_mappings[len]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => Some(
                &mut self.archetypes
                    [self.sorted_mappings[len][found_index].archetype_index as usize],
            ),
            Err(_) => None,
        };
    }

    // /// Returns mutable references to two different archetypes, creates the destination archetype if necessary.
    // /// Returns None if:
    // /// - any invalid archetype descriptor is provided
    // /// - destination archetype could not be created.
    // /// # Safety:
    // /// - Assumes source is a valid archetype and destination is as well.
    // /// - Assumes the are NOT identical.
    // pub(crate) unsafe fn find_or_create_archetypes_add_remove_unchecked(
    //     &mut self,
    //     source_archetype_index: u16,
    //     destination: &ArchetypeDescriptor,
    // ) -> Option<(&mut Archetype, &mut Archetype)> {
    //     // Safety: it's guaranteed source_archetype_index != destination_archetype_index, as they are known to be separate archetypes.
    //     let source_archetype = unsafe {
    //         &mut *self
    //             .archetypes
    //             .as_mut_ptr()
    //             .offset(source_archetype_index as isize)
    //     };
    //     let (_destination_arch_index, destination_archetype) =
    //         self.find_or_create_archetype(destination)?;
    //     debug_assert_ne!(source_archetype_index, _destination_arch_index);
    //
    //     Some((source_archetype, destination_archetype))
    // }

    pub(crate) fn find_or_create_archetype(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<(u16, &mut Archetype)> {
        let len = archetype_descriptor.len() as usize;
        if len > MAX_COMPONENTS_PER_ENTITY || !archetype_descriptor.is_valid() {
            return None;
        }
        return match self.sorted_mappings[len]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => Some((
                self.sorted_mappings[len][found_index].archetype_index,
                &mut self.archetypes
                    [self.sorted_mappings[len][found_index].archetype_index as usize],
            )),
            Err(insertion_index) => {
                if self.archetypes.len() >= MAX_ARCHETYPE_COUNT {
                    return None;
                }

                let archetype = Archetype::with_capacity(
                    archetype_descriptor,
                    DEFAULT_ARCHETYPE_ALLOCATION_SIZE as u32,
                );
                let key = SortedArchetypeKey {
                    id: archetype_descriptor.archetype_id(),
                    archetype_index: self.archetypes.len() as u16,
                };
                self.archetypes.push(archetype);
                self.sorted_mappings[len].insert(insertion_index, key);
                Some((
                    self.archetypes.len() as u16 - 1,
                    self.archetypes.last_mut().unwrap(),
                ))
            }
        };
    }
}

impl Index<u16> for ArchetypeRegistry {
    type Output = Archetype;

    fn index(&self, index: u16) -> &Self::Output {
        &self.archetypes[index as usize]
    }
}

impl IndexMut<u16> for ArchetypeRegistry {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.archetypes[index as usize]
    }
}
