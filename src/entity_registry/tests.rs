use super::registry::*;
use crate::*;

extern crate std;
use alloc::vec::Vec;

const ENTITY_COUNT: u32 = MAX_ENTITY_HANDLE_VALUE;

#[test]
fn test_entity_registry() {
    let mut registry = EntityRegistry::default();
    // Check destroying invalid entity.
    assert!(!registry.destroy_entity(Entity::invalid()));

    let entities = (0..ENTITY_COUNT)
        .into_iter()
        .filter_map(|_| registry.create_entity_with(1, 2))
        .collect::<Vec<_>>();
    entities.iter().rev().cloned().for_each(|e| {
        let entry = registry.get_entity_entry(e).unwrap();
        assert_eq!(entry.version(), 0);
        assert_eq!(entry.archetype_index(), 1);
        assert_eq!(entry.index_in_archetype().value(), 2);
        assert!(registry.destroy_entity(e));
    });
    entities.iter().cloned().for_each(|e| {
        assert_eq!(registry.destroy_entity(e), false);
        assert!(registry.get_entity_entry(e).is_none());
        assert!(registry.get_entity_entry_mut(e).is_none());
    });

    assert!(!registry.destroy_entity(Entity::invalid()));

    let entities = (0..ENTITY_COUNT)
        .into_iter()
        .filter_map(|_| registry.create_entity_with(1, 2))
        .collect::<Vec<_>>();
    entities.iter().rev().for_each(|entity| {
        let entry = registry.get_entity_entry(*entity).unwrap();
        assert_ne!(entry.version(), 0);
        assert_eq!(entry.archetype_index(), 1);
        assert_eq!(entry.index_in_archetype().value(), 2);
        assert!(registry.destroy_entity(*entity));
    });
    entities.iter().for_each(|entity| {
        assert!(!registry.destroy_entity(*entity));
        assert!(registry.get_entity_entry(*entity).is_none());
        assert!(registry.get_entity_entry_mut(*entity).is_none());
    });

    for entity in registry.iter() {
        let _ = entities.contains(&entity);
    }
}
