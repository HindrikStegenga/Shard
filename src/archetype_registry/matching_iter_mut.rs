use super::*;
use crate::archetype_registry::archetype_iter_mut::ArchetypeIterMut;
use crate::descriptors::component_group::ComponentGroup;
use crate::Entity;
use core::iter::FusedIterator;
use crate::archetype_registry::filter_clause::ComponentFilterGroup;

pub struct MatchingIterMut<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup = ()> {
    inner_iterator: ArchetypeIterMut<'a, G, F>,
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> MatchingIterMut<'a, G, F> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIterMut::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> Iterator for MatchingIterMut<'a, G, F> {
    type Item = G::SliceMutRefTuple;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> FusedIterator for MatchingIterMut<'a, G, F> {}

pub struct EntityMatchingIterMut<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup = ()> {
    inner_iterator: ArchetypeIterMut<'a, G, F>,
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> EntityMatchingIterMut<'a, G, F> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIterMut::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> Iterator for EntityMatchingIterMut<'a, G, F> {
    type Item = (&'a [Entity], G::SliceMutRefTuple);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>, F: ComponentFilterGroup> FusedIterator for EntityMatchingIterMut<'a, G, F> {}
