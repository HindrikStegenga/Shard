use super::super::*;
use super::*;
use crate::descriptors::component_group::ComponentGroup;
use crate::Entity;
use alloc::vec::*;
use core::iter::FusedIterator;

pub(crate) struct FilterMatchingIterMut<
    'a,
    G: ComponentGroup,
    F: Fn(&ArchetypeDescriptor) -> bool,
> {
    inner_iterator: FilterArchetypeIterMut<'a, G, F>,
}

impl<'a, G: ComponentGroup, F: Fn(&ArchetypeDescriptor) -> bool>
    FilterMatchingIterMut<'a, G, F>
{
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
        filter_closure: F,
    ) -> Self {
        Self {
            inner_iterator: FilterArchetypeIterMut::new(
                sorted_mappings,
                archetypes,
                filter_closure,
            ),
        }
    }
}

impl<'a, G: ComponentGroup, F: Fn(&ArchetypeDescriptor) -> bool> Iterator
    for FilterMatchingIterMut<'a, G, F>
{
    type Item = G::SliceMutRefTuple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup, F: Fn(&ArchetypeDescriptor) -> bool> FusedIterator
    for FilterMatchingIterMut<'a, G, F>
{
}

pub(crate) struct FilterEntityMatchingIterMut<
    'a,
    G: ComponentGroup,
    F: Fn(&ArchetypeDescriptor) -> bool,
> {
    inner_iterator: FilterArchetypeIterMut<'a, G, F>,
}

impl<'a, G: ComponentGroup, F: Fn(&ArchetypeDescriptor) -> bool>
    FilterEntityMatchingIterMut<'a, G, F>
{
    pub(in crate::archetype_registry) fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a mut [Archetype],
        filter_closure: F,
    ) -> Self {
        Self {
            inner_iterator: FilterArchetypeIterMut::new(
                sorted_mappings,
                archetypes,
                filter_closure,
            ),
        }
    }
}

impl<'a, G: ComponentGroup, F: Fn(&ArchetypeDescriptor) -> bool> Iterator
    for FilterEntityMatchingIterMut<'a, G, F>
{
    type Item = (&'a [Entity], G::SliceMutRefTuple<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let archetype = self.inner_iterator.next()?;
        unsafe { Some(archetype.get_entity_fuzzy_slices_unchecked_mut::<G>()) }
    }
}

impl<'a, G: ComponentGroup, F: Fn(&ArchetypeDescriptor) -> bool> FusedIterator
    for FilterEntityMatchingIterMut<'a, G, F>
{
}
