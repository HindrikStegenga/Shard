use super::*;
use crate::descriptors::component_group::ComponentGroup;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use crate::archetype_registry::filter_clause::ComponentFilterGroup;

pub(super) struct ArchetypeIter<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup = ()> {
    sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
    archetypes: &'a [Archetype],
    current_level: u8,
    current_index_in_level: usize,
    _phantom: PhantomData<fn(G, F)>,
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> ArchetypeIter<'a, G, F> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            sorted_mappings,
            archetypes,
            current_level: G::DESCRIPTOR.len() - 1,
            current_index_in_level: 0,
            _phantom: Default::default(),
        }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> Iterator for ArchetypeIter<'a, G, F> {
    type Item = &'a Archetype;

    fn next(&mut self) -> Option<Self::Item> {
        if !G::DESCRIPTOR.is_valid() {
            return None;
        }
        while (self.current_level as usize) < MAX_COMPONENTS_PER_ENTITY {
            unsafe {
                let level = &self
                    .sorted_mappings
                    .get_unchecked(self.current_level as usize);
                while (self.current_index_in_level as usize) < level.len() {
                    let arch_index = level
                        .get_unchecked(self.current_index_in_level as usize)
                        .archetype_index;
                    self.current_index_in_level += 1;
                    let archetype = &self.archetypes.get_unchecked(arch_index as usize);
                    if archetype
                        .descriptor()
                        .contains_subset(G::DESCRIPTOR.archetype()) && archetype.descriptor().excludes_subset(F::EXCLUSIONS)
                    {
                        return Some(archetype);
                    }
                }
                self.current_index_in_level = 0;
                self.current_level += 1;
            }
        }
        None
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> FusedIterator for ArchetypeIter<'a, G, F> {}
