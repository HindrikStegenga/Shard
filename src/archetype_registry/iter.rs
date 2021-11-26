use super::*;
use crate::component_group::ComponentGroup;
use core::marker::PhantomData;

pub struct ArchetypeIter<'a, G: ComponentGroup<'a>> {
    sorted_mappings: &'a [Vec<SortedArchetypeKey>; MAX_COMPONENTS_PER_ENTITY],
    archetypes: &'a [Archetype],
    current_level: u8,
    current_index_in_level: usize,
    _phantom: PhantomData<fn(G)>,
}

impl<'a, G: ComponentGroup<'a>> ArchetypeIter<'a, G> {
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

impl<'a, G: ComponentGroup<'a>> Iterator for ArchetypeIter<'a, G> {
    type Item = G::SliceRefTuple;

    fn next(&mut self) -> Option<Self::Item> {
        if !G::DESCRIPTOR.is_valid() {
            return None;
        }
        while (self.current_level as usize) < MAX_COMPONENTS_PER_ENTITY {
            if let Some(v) = {
                match self.sorted_mappings[self.current_index_in_level as usize]
                    .iter()
                    .enumerate()
                    .find_map(|(idx, key)| {
                        let arch = &self.archetypes[key.archetype_index as usize];
                        if arch.descriptor().contains(G::DESCRIPTOR.archetype()) {
                            Some((idx, arch))
                        } else {
                            None
                        }
                    }) {
                    Some((idx, arch)) => {
                        self.current_index_in_level = idx + 1;
                        Some(arch)
                    }
                    None => {
                        self.current_level += 1;
                        None
                    }
                }
            } {
                return Some(unsafe { v.get_fuzzy_slices_unchecked::<G>() });
            }
        }

        None
    }
}
