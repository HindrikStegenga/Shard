use super::*;
use crate::component_group::ComponentGroup;
use core::marker::PhantomData;

pub(super) struct ArchetypeIter<'a, G: ComponentGroup<'a>> {
    sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
    archetypes: &'a [Archetype],
    current_level: u8,
    current_index_in_level: usize,
    _phantom: PhantomData<fn(G)>,
}

impl<'a, G: ComponentGroup<'a>> ArchetypeIter<'a, G> {
    pub fn new(
        sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
        archetypes: &'a [Archetype],
    ) -> Self {
        Self {
            sorted_mappings,
            archetypes,
            current_level: G::DESCRIPTOR.len(),
            current_index_in_level: 0,
            _phantom: Default::default(),
        }
    }
}

impl<'a, G: ComponentGroup<'a>> Iterator for ArchetypeIter<'a, G> {
    type Item = G::SliceRefTuple;

    fn next(&mut self) -> Option<Self::Item> {
        if !G::DESCRIPTOR.is_valid() {
            return None;
        }

        None
    }
}
