use super::component_group::*;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_descriptor::ComponentDescriptor;
use crate::{copy_component_descriptor_from_to, ComponentTypeId};
use crate::{ArchetypeId, MAX_COMPONENTS_PER_ENTITY};

#[derive(Debug)]
pub struct ComponentGroupDescriptor {
    archetype: ArchetypeDescriptor,
    sorted_to_unsorted: [u8; MAX_COMPONENTS_PER_ENTITY],
    unsorted_to_sorted: [u8; MAX_COMPONENTS_PER_ENTITY],
}

impl ComponentGroupDescriptor {
    const INVALID: Self = Self {
        archetype: ArchetypeDescriptor::INVALID,
        sorted_to_unsorted: [0; MAX_COMPONENTS_PER_ENTITY],
        unsorted_to_sorted: [0; MAX_COMPONENTS_PER_ENTITY],
    };

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.archetype.is_valid()
    }

    #[inline(always)]
    pub fn archetype(&self) -> &ArchetypeDescriptor {
        &self.archetype
    }
    #[inline(always)]
    pub fn as_sorted(&self, index: u8) -> &ComponentDescriptor {
        &self.archetype.components()[index as usize]
    }
    #[inline(always)]
    pub fn as_unsorted(&self, index: u8) -> &ComponentDescriptor {
        &self.archetype.components()[self.sorted_to_unsorted[index as usize] as usize]
    }

    #[inline(always)]
    pub fn sorted_to_unsorted(&self, index: u8) -> u8 {
        self.sorted_to_unsorted[index as usize]
    }
    #[inline(always)]
    pub fn unsorted_to_sorted(&self, index: u8) -> u8 {
        self.unsorted_to_sorted[index as usize]
    }
    #[inline(always)]
    pub fn len(&self) -> u8 {
        self.archetype.len()
    }
}

impl ComponentGroupDescriptor {
    pub const fn new<const N: usize>(descriptors: &[ComponentDescriptor; N]) -> Self {
        if !Self::validate_component_descriptors(descriptors) {
            return Self::INVALID;
        }

        let sorted_descriptors = Self::compute_sorted_descriptors(descriptors);

        let id = ArchetypeDescriptor::compute_archetype_id(&sorted_descriptors);
        if !id.is_valid() {
            return Self::INVALID;
        }

        let (unsorted_to_sorted, sorted_to_unsorted) =
            ComponentGroupDescriptor::compute_sort_mappings(&descriptors, &sorted_descriptors);

        let value = Self {
            archetype: ArchetypeDescriptor::new(id, N as u8, sorted_descriptors),
            sorted_to_unsorted,
            unsorted_to_sorted,
        };
        if !value.archetype.archetype_id().is_valid() {
            return Self::INVALID;
        }
        value
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
    /// # Warning: only functions correctly if descriptors passed in are correctly validated.
    /// # Similarly, N must be smaller or equal to [`MAX_COMPONENTS_PER_ENTITY`].
    const fn compute_sorted_descriptors<const N: usize>(
        descriptors: &[ComponentDescriptor; N],
    ) -> [ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY] {
        let mut return_value = [ComponentDescriptor::INVALID; MAX_COMPONENTS_PER_ENTITY];
        let mut i = 0;
        while i < N {
            copy_component_descriptor_from_to!(descriptors[i], return_value[i]);
            i += 1;
        }
        i = 0;

        while i < N {
            let mut j = i + 1;
            while j < N {
                if return_value[j].component_type_id.into_u16()
                    < return_value[i].component_type_id.into_u16()
                {
                    let mut temp = ComponentDescriptor::INVALID;
                    copy_component_descriptor_from_to!(return_value[i], temp);
                    copy_component_descriptor_from_to!(return_value[j], return_value[i]);
                    copy_component_descriptor_from_to!(temp, return_value[j]);
                }
                j += 1;
            }
            i += 1;
        }

        return_value
    }

    /// Computes the mappings from sorted to unsorted and from unsorted to sorted.
    /// # Warning: MUST be used on valid mappings and N must be less than or equal to [`MAX_COMPONENTS_PER_ENTITY`].
    const fn compute_sort_mappings<const N: usize>(
        unsorted: &[ComponentDescriptor; N],
        sorted: &[ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY],
    ) -> (
        [u8; MAX_COMPONENTS_PER_ENTITY],
        [u8; MAX_COMPONENTS_PER_ENTITY],
    ) {
        let mut unsorted_to_sorted = [0; MAX_COMPONENTS_PER_ENTITY];
        let mut sorted_to_unsorted = [0; MAX_COMPONENTS_PER_ENTITY];

        let mut i = 0;
        while i < N {
            let mut j = 0;
            while j < N {
                if sorted[j].component_type_id.into_u16()
                    == unsorted[i].component_type_id.into_u16()
                {
                    unsorted_to_sorted[i] = j as u8;
                }
                if unsorted[j].component_type_id.into_u16()
                    == sorted[i].component_type_id.into_u16()
                {
                    sorted_to_unsorted[i] = j as u8;
                }
                j += 1;
            }
            i += 1;
        }
        (unsorted_to_sorted, sorted_to_unsorted)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::*;

    use super::*;
    use crate::{Component, ComponentTypeId};
    struct TestComponentA {}
    impl Component for TestComponentA {
        const NAME: &'static str = "A";
        const ID: ComponentTypeId = ComponentTypeId::from_u16(1);
    }
    struct TestComponentB {}
    impl Component for TestComponentB {
        const NAME: &'static str = "B";
        const ID: ComponentTypeId = ComponentTypeId::from_u16(2);
    }
    struct TestComponentC {}
    impl Component for TestComponentC {
        const NAME: &'static str = "C";
        const ID: ComponentTypeId = ComponentTypeId::from_u16(3);
    }

    #[test]
    fn test_compute_sorted_descriptors() {
        let descriptors: [ComponentDescriptor; 3] = [
            ComponentDescriptor::from_component::<TestComponentA>(),
            ComponentDescriptor::from_component::<TestComponentB>(),
            ComponentDescriptor::from_component::<TestComponentC>(),
        ];
        let result = ComponentGroupDescriptor::compute_sorted_descriptors(&descriptors);
        assert_eq!(
            ComponentDescriptor::from_component::<TestComponentA>(),
            result[0]
        );
        assert_eq!(
            ComponentDescriptor::from_component::<TestComponentB>(),
            result[1]
        );
        assert_eq!(
            ComponentDescriptor::from_component::<TestComponentC>(),
            result[2]
        );
        let descriptors: [ComponentDescriptor; 3] = [
            ComponentDescriptor::from_component::<TestComponentB>(),
            ComponentDescriptor::from_component::<TestComponentC>(),
            ComponentDescriptor::from_component::<TestComponentA>(),
        ];

        assert_eq!(
            ComponentDescriptor::from_component::<TestComponentA>(),
            result[0]
        );
        assert_eq!(
            ComponentDescriptor::from_component::<TestComponentB>(),
            result[1]
        );
        assert_eq!(
            ComponentDescriptor::from_component::<TestComponentC>(),
            result[2]
        );
    }

    #[test]
    fn test_compute_sort_mappings() {
        let unsorted_descriptors: [ComponentDescriptor; 3] = [
            ComponentDescriptor::from_component::<TestComponentA>(),
            ComponentDescriptor::from_component::<TestComponentB>(),
            ComponentDescriptor::from_component::<TestComponentC>(),
        ];
        let sorted_descriptors =
            ComponentGroupDescriptor::compute_sorted_descriptors(&unsorted_descriptors);

        let (unsorted_to_sorted, sorted_to_unsorted) =
            ComponentGroupDescriptor::compute_sort_mappings(
                &unsorted_descriptors,
                &sorted_descriptors,
            );
        assert_eq!(unsorted_to_sorted[0..3], [0, 1, 2]);
        assert_eq!(sorted_to_unsorted[0..3], [0, 1, 2]);

        let unsorted_descriptors: [ComponentDescriptor; 3] = [
            ComponentDescriptor::from_component::<TestComponentB>(),
            ComponentDescriptor::from_component::<TestComponentC>(),
            ComponentDescriptor::from_component::<TestComponentA>(),
        ];

        let sorted_descriptors =
            ComponentGroupDescriptor::compute_sorted_descriptors(&unsorted_descriptors);

        let (unsorted_to_sorted, sorted_to_unsorted) =
            ComponentGroupDescriptor::compute_sort_mappings(
                &unsorted_descriptors,
                &sorted_descriptors,
            );
        assert_eq!(unsorted_to_sorted[0..3], [1, 2, 0]);
        assert_eq!(sorted_to_unsorted[0..3], [2, 0, 1]);
    }
}
