use alloc::vec::Vec;

use crate::{
    component_descriptor::ComponentDescriptor, component_group::ComponentGroup,
    entity_registry::EntityRegistry, shard_registry::ShardRegistry, Component, Entity,
};

pub struct Registry {
    entities: EntityRegistry,
    shards: ShardRegistry,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            entities: EntityRegistry::default(),
            shards: ShardRegistry::default(),
        }
    }
}

impl Registry {
    pub fn create_entity<G: ComponentGroup>(&mut self, components: G) -> Result<Entity, G> {
        if G::LENGTH == 0 {
            return Err(components);
        };
        let entity_entry = match self.entities.create_entity() {
            Some(v) => v,
            None => return Err(components),
        };
        let (archetype, shard) = match self.shards.get_or_alloc_shard_from_group::<G>() {
            Some(v) => v,
            None => {
                let entity = entity_entry.entity();
                self.entities.destroy_entity(entity);
                return Err(components);
            }
        };
        //let entity = self.entities.create_entity(0, 0, G::LENGTH);
        todo!()
    }

    pub fn remove_entity(&mut self, entity: Entity) -> bool {
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

    pub fn replace_component<C1: Component, C2: Component>(
        &mut self,
        entity: Entity,
        new_component: C2,
    ) -> Result<C1, C2> {
        todo!()
    }
}
