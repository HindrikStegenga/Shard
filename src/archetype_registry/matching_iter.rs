use super::*;
use crate::archetype_registry::archetype_iter::ArchetypeIter;
use crate::component_group::ComponentGroup;
use crate::Entity;
use core::iter::FusedIterator;

pub struct MatchingIter<'a, G: ComponentGroup<'a>> {
    inner_iterator: ArchetypeIter<'a, G>,
}

impl<'a, G: ComponentGroup<'a>> MatchingIter<'a, G> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIter::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>> Iterator for MatchingIter<'a, G> {
    type Item = G::SliceRefTuple;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>> FusedIterator for MatchingIter<'a, G> {}

pub struct EntityMatchingIter<'a, G: ComponentGroup<'a>> {
    inner_iterator: ArchetypeIter<'a, G>,
}

impl<'a, G: ComponentGroup<'a>> EntityMatchingIter<'a, G> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIter::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>> Iterator for EntityMatchingIter<'a, G> {
    type Item = (&'a [Entity], G::SliceRefTuple);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>> FusedIterator for EntityMatchingIter<'a, G> {}
