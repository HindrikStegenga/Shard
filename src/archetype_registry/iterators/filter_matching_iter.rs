use super::super::*;
use super::*;
use crate::descriptors::component_group::ComponentGroup;
use crate::Entity;
use alloc::vec::*;
use core::iter::FusedIterator;

pub(crate) struct FilterMatchingIter<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool>
{
    inner_iterator: FilterArchetypeIter<'a, G, F>,
}

impl<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool> FilterMatchingIter<'a, G, F> {
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
        filter_closure: F,
    ) -> Self {
        Self {
            inner_iterator: FilterArchetypeIter::new(sorted_mappings, archetypes, filter_closure),
        }
    }
}

impl<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool> Iterator
    for FilterMatchingIter<'a, G, F>
{
    type Item = G::SliceRefTuple;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool> FusedIterator
    for FilterMatchingIter<'a, G, F>
{
}

pub(crate) struct FilterEntityMatchingIter<
    'a,
    G: ComponentGroup<'a>,
    F: Fn(&ArchetypeDescriptor) -> bool,
> {
    inner_iterator: FilterArchetypeIter<'a, G, F>,
}

impl<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool>
    FilterEntityMatchingIter<'a, G, F>
{
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
        filter_closure: F,
    ) -> Self {
        Self {
            inner_iterator: FilterArchetypeIter::new(sorted_mappings, archetypes, filter_closure),
        }
    }
}

impl<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool> Iterator
    for FilterEntityMatchingIter<'a, G, F>
{
    type Item = (&'a [Entity], G::SliceRefTuple);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked::<G>()) }
    }
}

impl<'a, G: ComponentGroup<'a>, F: Fn(&ArchetypeDescriptor) -> bool> FusedIterator
    for FilterEntityMatchingIter<'a, G, F>
{
}
