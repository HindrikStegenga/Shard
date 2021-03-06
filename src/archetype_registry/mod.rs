

mod sorted_archetype_key;

#[cfg(test)]
mod tests;
pub(crate) mod iterators;

use alloc::vec::*;
use core::ops::{Index, IndexMut};
use sorted_archetype_key::*;

use crate::archetype::Archetype;
use crate::descriptors::archetype_descriptor::ArchetypeDescriptor;
use crate::archetype_registry::iterators::*;
use crate::descriptors::component_descriptor::ComponentDescriptor;
use crate::descriptors::component_group::ComponentGroup;
use crate::constants::*;
use crate::Entity;

const DEFAULT_VECTOR_CAPACITY: usize = 64;

#[derive(Debug)]
/// Stores all archetypes.
pub struct ArchetypeRegistry {
    // TODO: Currently not a great approach, should become a graph
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
    pub fn find_archetype(
        &self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<&Archetype> {
        let len = archetype_descriptor.len() as usize;
        if len > MAX_COMPONENTS_PER_ENTITY || !archetype_descriptor.is_valid() {
            return None;
        }
        return match self.sorted_mappings[len - 1]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => Some(
                &self.archetypes
                    [self.sorted_mappings[len - 1][found_index].archetype_index as usize],
            ),
            Err(_) => None,
        };
    }

    #[allow(dead_code)]
    pub fn find_archetype_mut(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<&mut Archetype> {
        let len = archetype_descriptor.len() as usize;
        if len > MAX_COMPONENTS_PER_ENTITY || !archetype_descriptor.is_valid() {
            return None;
        }
        return match self.sorted_mappings[len - 1]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => Some(
                &mut self.archetypes
                    [self.sorted_mappings[len - 1][found_index].archetype_index as usize],
            ),
            Err(_) => None,
        };
    }

    /// Returns mutable reference to source archetype and finds or creates a new archetype by adding
    /// the given component type as defined by component descriptor.
    pub fn find_or_create_archetype_adding_component(
        &mut self,
        source_archetype_index: u16,
        component_descriptor: &ComponentDescriptor,
    ) -> Option<(&mut Archetype, u16, &mut Archetype)> {
        // Range check
        if source_archetype_index as usize > self.archetypes.len() {
            return None;
        }

        unsafe {
            // Safety: this pointer always is into self, and since we are adding a component to
            // the archetype descriptor, this means that the destination_archetype is always a different
            // one than the source archetype. As such, we can safely do this rather than needing to go
            // through split_at_mut() and remapping indices.
            let source_archetype: *mut Archetype = self
                .archetypes
                .get_unchecked_mut(source_archetype_index as usize);

            let new_archetype_descriptor = (*source_archetype)
                .descriptor()
                .add_component(component_descriptor)?;
            let (destination_archetype_index, destination_archetype) =
                self.find_or_create_archetype(&new_archetype_descriptor)?;

            Some((
                &mut *source_archetype,
                destination_archetype_index,
                destination_archetype,
            ))
        }
    }

    /// Returns mutable reference to source archetype and finds or creates a new archetype by removing
    /// the given component type as defined by component descriptor.
    pub fn find_or_create_archetype_removing_component(
        &mut self,
        source_archetype_index: u16,
        component_descriptor: &ComponentDescriptor,
    ) -> Option<(&mut Archetype, u16, &mut Archetype)> {
        // Range check
        if source_archetype_index as usize > self.archetypes.len() {
            return None;
        }

        unsafe {
            // Safety: this pointer always is into self, and since we are removing a component from
            // the archetype descriptor, this means that the destination_archetype is always a different
            // one than the source archetype. As such, we can safely do this rather than needing to go
            // through split_at_mut() and remapping indices.
            let source_archetype: *mut Archetype = self
                .archetypes
                .get_unchecked_mut(source_archetype_index as usize);

            let new_archetype_descriptor = (*source_archetype)
                .descriptor()
                .remove_component(component_descriptor.component_type_id())?;
            let (destination_archetype_index, destination_archetype) =
                self.find_or_create_archetype(&new_archetype_descriptor)?;

            Some((
                &mut *source_archetype,
                destination_archetype_index,
                destination_archetype,
            ))
        }
    }

    pub fn find_or_create_archetype(
        &mut self,
        archetype_descriptor: &ArchetypeDescriptor,
    ) -> Option<(u16, &mut Archetype)> {
        let len = archetype_descriptor.len() as usize;
        if len > MAX_COMPONENTS_PER_ENTITY || !archetype_descriptor.is_valid() {
            return None;
        }
        return match self.sorted_mappings[len - 1]
            .binary_search_by_key(&archetype_descriptor.archetype_id(), |e| e.id)
        {
            Ok(found_index) => Some((
                self.sorted_mappings[len - 1][found_index].archetype_index,
                &mut self.archetypes
                    [self.sorted_mappings[len - 1][found_index].archetype_index as usize],
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
                self.sorted_mappings[len - 1].insert(insertion_index, key);
                Some((
                    self.archetypes.len() as u16 - 1,
                    self.archetypes.last_mut().unwrap(),
                ))
            }
        };
    }

    pub unsafe fn get_unchecked(&self, index: u16) -> &Archetype {
        self.archetypes.get_unchecked(index as usize)
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: u16) -> &mut Archetype {
        self.archetypes.get_unchecked_mut(index as usize)
    }

    pub fn iter_components_matching<'a, G: ComponentGroup<'a>>(
        &'a self,
    ) -> impl Iterator<Item = <G as ComponentGroup<'a>>::SliceRefTuple> {
        MatchingIter::<'a, G>::new(&self.sorted_mappings, &self.archetypes)
    }

    pub fn iter_components_matching_mut<'a, G: ComponentGroup<'a>>(
        &'a mut self,
    ) -> impl Iterator<Item = <G as ComponentGroup<'a>>::SliceMutRefTuple> {
        MatchingIterMut::<'a, G>::new(&self.sorted_mappings, &mut self.archetypes)
    }

    pub fn iter_entity_components_matching<'a, G: ComponentGroup<'a>>(
        &'a self,
    ) -> impl Iterator<Item = (&'a [Entity], <G as ComponentGroup<'a>>::SliceRefTuple)> {
        EntityMatchingIter::<'a, G>::new(&self.sorted_mappings, &self.archetypes)
    }

    pub fn iter_entity_components_matching_mut<'a, G: ComponentGroup<'a>>(
        &'a mut self,
    ) -> impl Iterator<Item = (&'a [Entity], <G as ComponentGroup<'a>>::SliceMutRefTuple)> {
        EntityMatchingIterMut::<'a, G>::new(&self.sorted_mappings, &mut self.archetypes)
    }

    pub fn iter_filtered_components_matching<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool>(
        &'a self,
        filter_closure: F,
    ) -> impl Iterator<Item = <G as ComponentGroup<'a>>::SliceRefTuple> {
        FilterMatchingIter::<'a, G, F>::new(&self.sorted_mappings, &self.archetypes, filter_closure)
    }

    pub fn iter_filtered_components_matching_mut<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool>(
        &'a mut self,
        filter_closure: F,
    ) -> impl Iterator<Item = <G as ComponentGroup<'a>>::SliceMutRefTuple> {
        FilterMatchingIterMut::<'a, G, F>::new(&self.sorted_mappings, &mut self.archetypes, filter_closure)
    }

    pub fn iter_filtered_entity_components_matching<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool>(
        &'a self,
        filter_closure: F,
    ) -> impl Iterator<Item = (&'a [Entity], <G as ComponentGroup<'a>>::SliceRefTuple)> {
        FilterEntityMatchingIter::<'a, G, F>::new(&self.sorted_mappings, &self.archetypes, filter_closure)
    }

    pub fn iter_filtered_entity_components_matching_mut<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool>(
        &'a mut self,
        filter_closure: F,
    ) -> impl Iterator<Item = (&'a [Entity], <G as ComponentGroup<'a>>::SliceMutRefTuple)> {
        FilterEntityMatchingIterMut::<'a, G, F>::new(&self.sorted_mappings, &mut self.archetypes, filter_closure)
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
