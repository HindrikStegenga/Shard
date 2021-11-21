use alloc::vec::Vec;
use core::borrow::Borrow;

use crate::archetype::EntityMetadata;
use crate::archetype_registry::ArchetypeRegistry;
use crate::component_group_descriptor::ComponentGroupDescriptor;
use crate::{
    component_descriptor::ComponentDescriptor, component_group::ComponentGroup,
    entity_registry::EntityRegistry, Component, Entity,
};

mod tests;

pub struct Registry {
    entities: EntityRegistry,
    archetypes: ArchetypeRegistry,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            entities: EntityRegistry::default(),
            archetypes: ArchetypeRegistry::default(),
        }
    }
}

impl Registry {
    pub fn create_entity<'c, G: ComponentGroup<'c>>(&mut self, components: G) -> Result<Entity, G> {
        if !G::DESCRIPTOR.is_valid() {
            return Err(components);
        }

        let mut entity_entry = match self.entities.create_entity() {
            Some(v) => v,
            None => return Err(components),
        };
        let (archetype_index, archetype) = match self
            .archetypes
            .find_or_create_archetype(G::DESCRIPTOR.archetype())
        {
            Some(v) => v,
            None => return Err(components),
        };
        let metadata = EntityMetadata::new(entity_entry.entity());
        let idx_in_archetype = unsafe { archetype.push_entity_unchecked(metadata, components) };
        entity_entry.set_index_in_archetype(idx_in_archetype);
        entity_entry.set_archetype_index(archetype_index);
        Ok(entity_entry.entity())
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        let entry = match self.entities.get_entity_entry(entity) {
            None => return false,
            Some(v) => v,
        };
        let mut archetype = &mut self.archetypes[entry.archetype_index()];
        let index_in_archetype = entry.index_in_archetype();
        unsafe {
            if archetype.swap_drop_unchecked(index_in_archetype) {
                // A swap was needed, so we need to update the index_in_archetype of the entry that it was swapped with.
                // We retrieve the entity handle using the metadata, which is now at the old entity's position.
                let swapped_entity =
                    archetype.entity_metadata()[index_in_archetype as usize].entity();
                self.entities
                    .get_entity_entry_mut(swapped_entity)
                    .unwrap()
                    .set_index_in_archetype(index_in_archetype);
            }
        };
        self.entities.destroy_entity(entity);
        true
    }

    pub fn has_component<C: Component>(&self, entity: Entity) -> bool {
        let entry = match self.entities.get_entity_entry(entity) {
            None => return false,
            Some(v) => v,
        };
        let archetype = &self.archetypes[entry.archetype_index()];
        archetype.descriptor().has_component::<C>()
    }

    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        let entry = match self.entities.get_entity_entry(entity) {
            None => return None,
            Some(v) => v,
        };
        unsafe {
            self.archetypes[entry.archetype_index()]
                .get_component_unchecked::<C>(entry.index_in_archetype())
        }
        .into()
    }

    pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        let entry = match self.entities.get_entity_entry(entity) {
            None => return None,
            Some(v) => v,
        };
        unsafe {
            self.archetypes[entry.archetype_index()]
                .get_component_unchecked_mut::<C>(entry.index_in_archetype())
        }
        .into()
    }

    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Result<(), C> {
        todo!()
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Result<C, ()> {
        todo!()
    }

    pub fn replace_component<C1: Component, C2: Component>(
        &mut self,
        entity: Entity,
        new_component: C2,
    ) -> Result<C1, C2> {
        todo!()
    }
}
