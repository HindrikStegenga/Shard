use super::super::*;
use super::*;
use crate::descriptors::component_group::ComponentGroup;
use crate::Entity;
use alloc::vec::*;
use core::iter::FusedIterator;

pub(crate) struct MatchingIter<'a, G: ComponentGroup> {
    inner_iterator: ArchetypeIter<'a, G>,
}

impl<'a, G: ComponentGroup> MatchingIter<'a, G> {
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIter::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup> Iterator for MatchingIter<'a, G> {
    type Item = G::SliceRefTuple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup> FusedIterator for MatchingIter<'a, G> {}

pub(crate) struct EntityMatchingIter<'a, G: ComponentGroup> {
    inner_iterator: ArchetypeIter<'a, G>,
}

impl<'a, G: ComponentGroup> EntityMatchingIter<'a, G> {
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIter::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup> Iterator for EntityMatchingIter<'a, G> {
    type Item = (&'a [Entity], G::SliceRefTuple<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup> FusedIterator for EntityMatchingIter<'a, G> {}
