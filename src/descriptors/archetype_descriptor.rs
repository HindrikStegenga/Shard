use crate::descriptors::component_descriptor::ComponentDescriptor;
use crate::fnv1a::fnv1a_hash_32;
use crate::{constants::*, ArchetypeId, Component, ComponentTypeId};

/// Represents a combination of components.
/// Each component type MUST be unique (i.e. no duplicate component types).
/// Length must be larger than 0 and lower or equal to [`MAX_COMPONENTS_PER_ENTITY`].
/// Use the [`ArchetypeDescriptor::is_valid`] function to check for validity.
/// Any use of an invalid archetype descriptor is considered UB.
#[derive(Debug, Clone)]
pub struct ArchetypeDescriptor {
    archetype_id: ArchetypeId,
    components: [ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY],
    len: u8,
}

impl ArchetypeDescriptor {
    /// The invalid archetype descriptor. Has 0 components and an ArchetypeId of 0.
    pub const INVALID: ArchetypeDescriptor = ArchetypeDescriptor {
        archetype_id: ArchetypeId::INVALID,
        len: 0,
        components: [ComponentDescriptor::INVALID; MAX_COMPONENTS_PER_ENTITY],
    };

    /// Returns true if it is a valid archetype.
    /// A valid archetype has a length larger than 0 and smaller than [`MAX_COMPONENTS_PER_ENTITY`].
    /// It also contains no duplicate components.
    pub const fn is_valid(&self) -> bool {
        self.archetype_id.is_valid()
    }

    /// Creates a new archetype descriptor with the given id, length and components.
    pub const fn new(
        archetype_id: ArchetypeId,
        len: u8,
        components: [ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY],
    ) -> Self {
        if len == 0 || !archetype_id.is_valid() {
            return Self::INVALID;
        }
        Self {
            archetype_id,
            len,
            components,
        }
    }

    /// Computes an archetype ID, returns [`ArchetypeId::INVALID`] if given an invalid combination of components.
    pub const fn compute_archetype_id(descriptors: &[ComponentDescriptor]) -> ArchetypeId {
        if descriptors.is_empty() {
            return ArchetypeId::INVALID;
        }
        if descriptors.len() == 1 {
            return ArchetypeId::from_u32(descriptors[0].component_type_id().into_u16() as u32);
        }

        let mut bytes = [0; MAX_COMPONENTS_PER_ENTITY * core::mem::size_of::<ComponentTypeId>()];
        let mut i = 0;
        while i < descriptors.len() {
            let byte_block = ComponentTypeId::to_ne_bytes(descriptors[i].component_type_id());
            let mut j = 0;
            while j < core::mem::size_of::<ComponentTypeId>() {
                bytes[i * core::mem::size_of::<ComponentTypeId>() + j] = byte_block[j];
                j += 1;
            }
            i += 1;
        }
        ArchetypeId::from_u32(fnv1a_hash_32(&bytes, Some(bytes.len())))
    }

    /// Returns whether the descriptor provided is contained in self. (i.e. subset inclusion)
    /// Do not provide an invalid descriptor to this!
    pub const fn contains_subset(&self, descriptor: &ArchetypeDescriptor) -> bool {
        if descriptor.len() > self.len() {
            return false;
        }
        let mut i = 0;
        'outer_loop: while i < descriptor.len() {
            let mut j = 0;
            while j < self.len() {
                if self.components[j as usize].component_type_id.into_u16()
                    == descriptor.components[i as usize]
                        .component_type_id
                        .into_u16()
                {
                    i += 1;
                    continue 'outer_loop;
                }
                j += 1;
            }
            return false;
        }
        return true;
    }

    /// Returns whether the descriptor provided is excluded from self. (i.e. subset exclusion)
    /// Do not provide an invalid descriptor to this!
    pub const fn excludes_subset(&self, descriptor: &ArchetypeDescriptor) -> bool {
        let mut i = 0;
        while i < descriptor.len() {
            let mut j = 0;
            while j < self.len() {
                if self.components[j as usize].component_type_id.into_u16()
                    == descriptor.components[i as usize]
                    .component_type_id
                    .into_u16()
                {
                    return false;
                }
                j += 1;
            }
            i += 1;
        }
        return true;
    }

    /// Returns a new archetype with the given component type added to it.
    /// Returns none if the current archetype already contains the component type or it is full.
    #[allow(dead_code)]
    pub fn add_component_from<C: Component>(&self) -> Option<ArchetypeDescriptor> {
        self.add_component(&C::DESCRIPTOR)
    }

    /// Returns a new archetype with the given component type added to it.
    /// Returns none if the current archetype already contains the component type or it is full.
    pub fn add_component(
        &self,
        component_descriptor: &ComponentDescriptor,
    ) -> Option<ArchetypeDescriptor> {
        if self.len() as usize == MAX_COMPONENTS_PER_ENTITY {
            return None; // Archetype is full.
        }
        match self.components[0..self.len() as usize]
            .binary_search_by_key(&component_descriptor.component_type_id, |e| {
                e.component_type_id
            }) {
            Ok(_) => None, // Current archetype already contains given component.
            Err(insertion_index) => {
                let mut v = self.clone();
                for i in insertion_index..self.len() as usize + 1 {
                    v.components[i + 1] = self.components[i].clone();
                }
                v.components[insertion_index] = component_descriptor.clone();

                v.len += 1;
                v.archetype_id =
                    ArchetypeDescriptor::compute_archetype_id(&v.components[0..v.len() as usize]);
                Some(v)
            }
        }
    }

    pub fn remove_component(
        &self,
        component: ComponentTypeId,
    ) -> Option<ArchetypeDescriptor> {
        if self.len() as usize == 1 {
            return None; // Archetype cannot contain zero components.
        }
        match self.components[0..self.len() as usize]
            .binary_search_by_key(&component, |e| e.component_type_id)
        {
            Ok(found_index) => {
                let mut v = self.clone();
                for i in found_index..self.len() as usize {
                    v.components[i] = self.components[i + 1].clone();
                }

                v.len -= 1;
                v.archetype_id =
                    ArchetypeDescriptor::compute_archetype_id(&v.components[0..v.len() as usize]);
                Some(v)
            }
            Err(_) => None,
        }
    }

    /// Returns whether the archetype descriptor has a given component type.
    // TODO: Check if this can be constified somehow?
    pub fn has_component<C: Component>(&self) -> bool {
        return match self
            .components()
            .binary_search_by_key(&C::ID, |e| e.component_type_id)
        {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    /// Get a the archetype descriptor's archetype id.
    pub const fn archetype_id(&self) -> ArchetypeId {
        self.archetype_id
    }

    /// Get the archetype descriptor's component count.
    pub const fn len(&self) -> u8 {
        self.len
    }

    /// Get a reference to the archetype descriptor's components.
    /// This version is const but unsafe, as length is NOT accounted for.
    pub const unsafe fn components_unchecked(
        &self,
    ) -> &[ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY] {
        &self.components
    }

    /// Get a reference to the archetype descriptor's components.
    pub fn components(&self) -> &[ComponentDescriptor] {
        &self.components[0..self.len as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::descriptors::archetype_descriptor::ArchetypeDescriptor;
    use crate::descriptors::component_group::ComponentGroup;
    use crate::test_components::*;
    use crate::Component;

    #[test]
    fn test_archetype_descriptor_add_remove() {
        let descriptor: &ArchetypeDescriptor = <(A, B) as ComponentGroup>::DESCRIPTOR.archetype();
        assert_eq!(descriptor.has_component::<A>(), true);
        assert_eq!(descriptor.has_component::<B>(), true);
        let descriptor = descriptor
            .add_component(&<C as Component>::DESCRIPTOR)
            .unwrap();
        assert_eq!(descriptor.has_component::<C>(), true);
        assert_eq!(descriptor.has_component::<A>(), true);
        assert_eq!(descriptor.has_component::<B>(), true);
        let descriptor = descriptor.remove_component(B::ID).unwrap();
        assert_eq!(descriptor.has_component::<B>(), false);
        assert_eq!(descriptor.has_component::<A>(), true);
        assert_eq!(descriptor.has_component::<C>(), true);
        assert_eq!(descriptor.len(), 2);
    }

    #[test]
    fn test_archetype_descriptor_contains() {
        assert_eq!(
            <(A, B) as ComponentGroup>::DESCRIPTOR
                .archetype()
                .contains_subset(<A as ComponentGroup>::DESCRIPTOR.archetype()),
            true
        );
        assert_eq!(
            <(A, B) as ComponentGroup>::DESCRIPTOR
                .archetype()
                .contains_subset(<B as ComponentGroup>::DESCRIPTOR.archetype()),
            true
        );
        assert_eq!(
            <(A, B) as ComponentGroup>::DESCRIPTOR
                .archetype()
                .contains_subset(<C as ComponentGroup>::DESCRIPTOR.archetype()),
            false
        );
    }

    #[test]
    fn test_archetype_descriptor_excludes() {
        assert_eq!(
            <(A, B) as ComponentGroup>::DESCRIPTOR
                .archetype()
                .excludes_subset(<C as ComponentGroup>::DESCRIPTOR.archetype()),
            true
        );
        assert_eq!(
            <(A, B) as ComponentGroup>::DESCRIPTOR
                .archetype()
                .excludes_subset(<A as ComponentGroup>::DESCRIPTOR.archetype()),
            false
        );
        assert_eq!(
            <(A, B) as ComponentGroup>::DESCRIPTOR
                .archetype()
                .contains_subset(<B as ComponentGroup>::DESCRIPTOR.archetype()),
            true
        );
    }
}
