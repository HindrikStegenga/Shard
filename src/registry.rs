use alloc::vec::Vec;

use crate::{
    component_descriptor::ComponentDescriptor, entity_registry::EntityRegistry, Component, Entity,
};

pub struct Registry {
    entities: EntityRegistry,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            entities: EntityRegistry::default(),
        }
    }
}

impl Registry {
    pub fn create_entity_from_component<C: Component>(&mut self) -> Entity {
        todo!()
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        todo!()
    }

    pub fn has_component<C: Component>(&self, entity: Entity) -> bool {
        todo!()
    }

    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        todo!()
    }

    pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        todo!()
    }

    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Result<(), C> {
        todo!()
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Result<C, ()> {
        todo!()
    }
}
