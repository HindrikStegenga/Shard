use super::component_group::*;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_descriptor::ComponentDescriptor;
use crate::copy_from_component_descriptor;
use crate::{ArchetypeId, MAX_COMPONENTS_PER_ENTITY};

#[derive(Debug)]
pub struct ComponentGroupDescriptor {
    archetype: ArchetypeDescriptor,
    sorted_to_unsorted: [u8; MAX_COMPONENTS_PER_ENTITY],
    unsorted_to_sorted: [u8; MAX_COMPONENTS_PER_ENTITY],
}

impl ComponentGroupDescriptor {
    pub fn archetype(&self) -> &ArchetypeDescriptor {
        &self.archetype
    }
    pub fn sorted_to_unsorted(&self) -> [u8; MAX_COMPONENTS_PER_ENTITY] {
        self.sorted_to_unsorted
    }
    pub fn unsorted_to_sorted(&self) -> [u8; MAX_COMPONENTS_PER_ENTITY] {
        self.unsorted_to_sorted
    }
}

impl ComponentGroupDescriptor {
    pub const fn new<const N: usize>(descriptors: [ComponentDescriptor; N]) -> Option<Self> {
        if !Self::validate_component_descriptors(&descriptors) {
            return None;
        }

        let sorted_descriptors = Self::compute_sorted_descriptors(descriptors);

        let id = ArchetypeDescriptor::compute_archetype_id(&sorted_descriptors);
        if !id.is_valid() {
            return None;
        }

        let value = Self {
            archetype: ArchetypeDescriptor::new(id, N as u8, sorted_descriptors),
            sorted_to_unsorted: [0; MAX_COMPONENTS_PER_ENTITY],
            unsorted_to_sorted: [0; MAX_COMPONENTS_PER_ENTITY],
        };
        if !value.archetype.archetype_id().is_valid() {}
        Some(value)
    }

    const fn validate_component_descriptors<const N: usize>(
        descriptors: &[ComponentDescriptor; N],
    ) -> bool {
        // Length may not be zero or larger than max components.
        if descriptors.len() == 0 || descriptors.len() > MAX_COMPONENTS_PER_ENTITY {
            return false;
        }
        // Duplicates MUST not exist.
        let mut idx = 0;
        while idx < N {
            let mut cdx = idx + 1;
            while cdx < N {
                if descriptors[idx].component_type_id().into_u16()
                    == descriptors[cdx].component_type_id().into_u16()
                {
                    return false;
                }
                cdx += 1;
            }
            idx += 1;
        }
        true
    }

    /// Computes the sorted version of a given array of descriptors.
    /// # Warning: only functions correctly if descriptors passed in are correctly validated!!
    const fn compute_sorted_descriptors<const N: usize>(
        mut descriptors: [ComponentDescriptor; N],
    ) -> [ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY] {
        let mut return_value = [ComponentDescriptor::INVALID; MAX_COMPONENTS_PER_ENTITY];
        let mut tmp = ComponentDescriptor::INVALID;
        // let mut i = 0;
        // while i < N {
        //     let mut pos = i;
        //     let mut j = i + 1;
        //     while j < N {
        //         if descriptors[i].component_type_id.into_u16()
        //             > descriptors[j].component_type_id.into_u16()
        //         {
        //             pos = j;
        //         }
        //         copy_from_component_descriptor!(tmp, descriptors[i]);
        //         copy_from_component_descriptor!(descriptors[i], descriptors[pos]);
        //         copy_from_component_descriptor!(descriptors[i], descriptors[pos]);
        //         j += 1;
        //     }
        //     i += 1;
        // }
        // i = 0;
        // while i < N {
        //     copy_from_component_descriptor!(return_value[i], descriptors[i]);
        //     i += 1;
        // }

        todo!("Fix all this stuff here");

        return_value
    }
}
