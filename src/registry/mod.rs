use crate::archetype::{Archetype, EntityMetadata};
use crate::archetype_registry::ArchetypeRegistry;
use crate::entity_registry::EntityEntry;
use crate::{component_group::ComponentGroup, entity_registry::EntityRegistry, Component, Entity};

#[cfg(test)]
mod tests;

/// The primary construct in the *Shard* Entity Component System (ECS).
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
    /// Creates a new entity using the provided components.
    /// Returns Ok with a Entity if successful, or Err(components) if not.
    /// Returns Err if the provided component group is invalid, an internal limit is exceeded.
    /// Panics in case of allocation failure.
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

    /// Removes the entity from the registry.
    /// This function return false if the entity given is invalid.
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
        let _v = self.entities.destroy_entity(entity);
        debug_assert!(_v);
        true
    }

    /// Removes the entity from the registry if it matches the given component group exactly.
    /// Otherwise, it simply leaves the entity as is.
    /// This function return None if either entity given is invalid, or does not match the given component group.
    pub fn remove_entity<'a, G: ComponentGroup<'a>>(&'a mut self, entity: Entity) -> Option<G> {
        let entry = match self.entities.get_entity_entry(entity) {
            None => return None,
            Some(v) => v,
        };
        let archetype = &mut self.archetypes[entry.archetype_index()];
        let index_in_archetype = entry.index_in_archetype();
        unsafe {
            return match archetype.swap_remove_unchecked::<G>(index_in_archetype) {
                (value, true) => {
                    // A swap was needed, so we need to update the index_in_archetype of the entry that it was swapped with.
                    // We retrieve the entity handle using the metadata, which is now at the old entity's position.
                    let swapped_entity =
                        archetype.entity_metadata()[index_in_archetype as usize].entity();
                    self.entities
                        .get_entity_entry_mut(swapped_entity)
                        .unwrap()
                        .set_index_in_archetype(index_in_archetype);
                    Some(value)
                }
                (value, false) => Some(value),
            };
        }
    }

    /// Returns true if a given entity has the specified component.
    /// Returns false if entity is invalid or does not have the specified component.
    pub fn has_component<C: Component>(&self, entity: Entity) -> bool {
        let entry = match self.entities.get_entity_entry(entity) {
            None => return false,
            Some(v) => v,
        };
        let archetype = &self.archetypes[entry.archetype_index()];
        archetype.descriptor().has_component::<C>()
    }

    /// Returns a reference to the specified component if the entity has it.
    /// Returns false if entity is invalid or does not have the specified component.
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

    /// Returns a mutable reference to the specified component if the entity has it.
    /// Returns false if entity is invalid or does not have the specified component.
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
        let entry = match self.entities.get_entity_entry(entity) {
            None => return Err(component),
            Some(v) => v.clone(),
        };

        let mut source_archetype =
            unsafe { &mut (*(self as *mut Self)).archetypes[entry.archetype_index()] };
        let mut new_descriptor = match source_archetype.descriptor().clone().add_component::<C>() {
            None => return Err(component),
            Some(v) => v,
        };
        let (_, destination_arch) = match self.archetypes.find_or_create_archetype(&new_descriptor)
        {
            None => {
                return Err(component);
            }
            Some(v) => v,
        };

        unsafe {
            Archetype::copy_common_components_between_archetypes_unchecked(
                source_archetype,
                entry.index_in_archetype(),
                destination_arch,
                0,
            );
        }
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
