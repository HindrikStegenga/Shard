use super::*;
use crate::archetype_registry::archetype_iter_mut::ArchetypeIterMut;
use crate::descriptors::component_group::ComponentGroup;
use crate::Entity;
use core::iter::FusedIterator;

pub struct MatchingIterMut<'a, G: ComponentGroup<'a>> {
    inner_iterator: ArchetypeIterMut<'a, G>,
}

impl<'a, G: ComponentGroup<'a>> MatchingIterMut<'a, G> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIterMut::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>> Iterator for MatchingIterMut<'a, G> {
    type Item = G::SliceMutRefTuple;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>> FusedIterator for MatchingIterMut<'a, G> {}

pub struct EntityMatchingIterMut<'a, G: ComponentGroup<'a>> {
    inner_iterator: ArchetypeIterMut<'a, G>,
}

impl<'a, G: ComponentGroup<'a>> EntityMatchingIterMut<'a, G> {
    pub(super) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIterMut::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup<'a>> Iterator for EntityMatchingIterMut<'a, G> {
    type Item = (&'a [Entity], G::SliceMutRefTuple);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>> FusedIterator for EntityMatchingIterMut<'a, G> {}
