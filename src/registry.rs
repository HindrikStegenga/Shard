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
    fn create_entity_from_component<C: Component>(&mut self) -> Entity {
        todo!()
    }

    fn destroy_entity(&mut self, entity: Entity) {
        todo!()
    }
}
