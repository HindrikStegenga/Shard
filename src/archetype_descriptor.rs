use crate::component_descriptor::ComponentDescriptor;
use crate::fnv1a::fnv1a_hash_32;
use crate::{constants::*, ArchetypeId, Component, ComponentTypeId};

#[derive(Debug, Clone)]
pub struct ArchetypeDescriptor {
    archetype_id: ArchetypeId,
    components: [ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY],
    len: u8,
}

impl ArchetypeDescriptor {
    pub const INVALID: ArchetypeDescriptor = ArchetypeDescriptor {
        archetype_id: ArchetypeId::INVALID,
        len: 0,
        components: [ComponentDescriptor::INVALID; MAX_COMPONENTS_PER_ENTITY],
    };

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

    pub(crate) const fn compute_archetype_id(descriptors: &[ComponentDescriptor]) -> ArchetypeId {
        if descriptors.is_empty() {
            return ArchetypeId::from_u32(0);
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

    /// Returns a new archetype with the given component type added to it.
    /// Returns none if the current archetype already contains the component type or it is full.
    pub(crate) fn add_component<C: Component>(&self) -> Option<ArchetypeDescriptor> {
        if self.len as usize == MAX_COMPONENTS_PER_ENTITY || self.len == 1 {
            return None; // Archetype is full.
        }
        match self.components[0..self.len as usize]
            .binary_search_by_key(&C::ID, |e| e.component_type_id)
        {
            Ok(_) => None, // Current archetype already contains given component.
            Err(insertion_index) => {
                let mut v = self.clone();
                for i in insertion_index..self.len as usize + 1 {
                    v.components[i + 1] = v.components()[i].clone();
                }
                v.components[insertion_index] = ComponentDescriptor::from_component::<C>();

                v.len += 1;
                v.archetype_id =
                    ArchetypeDescriptor::compute_archetype_id(&v.components[0..v.len() as usize]);
                Some(v)
            }
        }
    }
    /// Get a the archetype descriptor's archetype id.
    pub const fn archetype_id(&self) -> ArchetypeId {
        self.archetype_id
    }

    /// Get the archetype descriptor's component count.
    pub fn len(&self) -> u8 {
        self.len
    }

    /// Get a reference to the archetype descriptor's components.
    pub fn components(&self) -> &[ComponentDescriptor] {
        &self.components[0..self.len as usize]
    }
}
