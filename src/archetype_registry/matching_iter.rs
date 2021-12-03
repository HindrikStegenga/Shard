use super::*;
use crate::archetype_registry::archetype_iter::ArchetypeIter;
use crate::descriptors::component_group::ComponentGroup;
use crate::Entity;
use core::iter::FusedIterator;
use crate::archetype_registry::filter_clause::ComponentFilterGroup;

pub struct MatchingIter<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup = ()> {
    inner_iterator: ArchetypeIter<'a, G, F>,
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> MatchingIter<'a, G, F> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIter::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> Iterator for MatchingIter<'a, G, F> {
    type Item = G::SliceRefTuple;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> FusedIterator for MatchingIter<'a, G, F> {}

pub struct EntityMatchingIter<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup = ()> {
    inner_iterator: ArchetypeIter<'a, G, F>,
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> EntityMatchingIter<'a, G, F> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIter::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> Iterator for EntityMatchingIter<'a, G, F> {
    type Item = (&'a [Entity], G::SliceRefTuple);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> FusedIterator for EntityMatchingIter<'a, G, F> {}
