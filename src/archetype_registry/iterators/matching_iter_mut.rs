use super::super::*;
use super::*;
use crate::descriptors::component_group::ComponentGroup;
use crate::Entity;
use alloc::vec::*;
use core::iter::FusedIterator;

pub(crate) struct MatchingIterMut<'a, G: ComponentGroup> {
    inner_iterator: ArchetypeIterMut<'a, G>,
}

impl<'a, G: ComponentGroup> MatchingIterMut<'a, G> {
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIterMut::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup> Iterator for MatchingIterMut<'a, G> {
    type Item = G::SliceMutRefTuple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup> FusedIterator for MatchingIterMut<'a, G> {}

pub(crate) struct EntityMatchingIterMut<'a, G: ComponentGroup> {
    inner_iterator: ArchetypeIterMut<'a, G>,
}

impl<'a, G: ComponentGroup> EntityMatchingIterMut<'a, G> {
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
    ) -> Self {
        Self {
            inner_iterator: ArchetypeIterMut::new(sorted_mappings, archetypes),
        }
    }
}

impl<'a, G: ComponentGroup> Iterator for EntityMatchingIterMut<'a, G> {
    type Item = (&'a [Entity], G::SliceMutRefTuple<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup> FusedIterator for EntityMatchingIterMut<'a, G> {}
