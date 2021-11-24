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

    /// Adds a given component to the entity if it's not yet present.
    /// Returns the original component in case of failure for any reason.
    /// Reasons for failure:
    /// - Invalid entity provided.
    /// - Destination archetype could not be created.
    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Result<(), C> {
        let entry = match self.entities.get_entity_entry(entity) {
            None => return Err(component),
            Some(v) => v.clone(),
        };

        // Get the new archetype
        let (source_archetype, destination_archetype_index, destination_archetype) = match self
            .archetypes
            .find_or_create_archetype_adding_component(entry.archetype_index(), &C::DESCRIPTOR)
        {
            Some(v) => v,
            None => return Err(component),
        };

        // Make sure the entity we move is at the end of it's archetype (so data stays contiguous).
        if unsafe { source_archetype.swap_to_last_unchecked(entry.index_in_archetype()) } {
            // A swap was needed, so we need to update the index_in_archetype of the entry that it was swapped with.
            // We retrieve the entity handle using the metadata, which is now at the swapped with entity's position.
            let swapped_entity =
                source_archetype.entity_metadata()[entry.index_in_archetype() as usize].entity();
            self.entities
                .get_entity_entry_mut(swapped_entity)
                .unwrap()
                .set_index_in_archetype(entry.index_in_archetype());
        }

        // copy from end to end.
        let new_source_entity_index_in_archetype = source_archetype.len() - 1;
        let destination_entity_index_in_archetype = destination_archetype.len() - 1;

        unsafe {
            // Make space in the destination archetype.
            destination_archetype.push_uninitialized_entity();
            // Write common components.
            Archetype::copy_common_components_between_archetypes_unchecked(
                source_archetype,
                new_source_entity_index_in_archetype,
                destination_archetype,
                destination_entity_index_in_archetype,
            );
            // Write added component
            destination_archetype
                .write_single_component_unchecked(destination_entity_index_in_archetype, component);

            // Copy the metadata
            destination_archetype.entity_metadata_mut()
                [destination_entity_index_in_archetype as usize] =
                source_archetype.entity_metadata()[new_source_entity_index_in_archetype as usize];

            // Make the source archetype forget the old entity.
            source_archetype.decrement_len_unchecked();
        }

        // Update the original entity entry to point to destination archetype and index in archetype.
        let entity_entry = self.entities.get_entity_entry_mut(entity).unwrap();
        entity_entry.set_archetype_index(destination_archetype_index);
        entity_entry.set_index_in_archetype(destination_entity_index_in_archetype);

        Ok(())
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