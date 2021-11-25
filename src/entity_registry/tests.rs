use super::*;
extern crate std;

#[test]
fn test_entity_registry() {
    let mut registry = EntityRegistry::default();

    assert!(!registry.destroy_entity(Entity::invalid()));

    let valid_entities = (0..MAX_ENTITY_HANDLE_VALUE)
        .into_iter()
        .filter_map(|_| registry.create_entity_with(1, 2))
        .collect::<Vec<_>>();
    valid_entities.iter().rev().for_each(|entity| {
        let entry = registry.get_entity_entry(*entity).unwrap();
        assert_eq!(entry.version(), 0);
        assert_eq!(entry.archetype_index(), 1);
        assert_eq!(entry.index_in_archetype(), 2);
        assert!(registry.destroy_entity(*entity));
    });
    valid_entities.iter().for_each(|entity| {
        assert_eq!(registry.destroy_entity(*entity), false);
        assert!(registry.get_entity_entry(*entity).is_none());
        assert!(registry.get_entity_entry_mut(*entity).is_none());
    });

    assert!(!registry.destroy_entity(Entity::invalid()));

    let valid_entities = (0..MAX_ENTITY_HANDLE_VALUE)
        .into_iter()
        .filter_map(|_| {
            let mut e = registry.create_entity()?;
            e.set_archetype_index(1);
            e.set_index_in_archetype(2);
            Some(e.entity())
        })
        .collect::<Vec<_>>();
    valid_entities.iter().rev().for_each(|entity| {
        let entry = registry.get_entity_entry(*entity).unwrap();
        assert_eq!(entry.version(), 0);
        assert_eq!(entry.archetype_index(), 1);
        assert_eq!(entry.index_in_archetype(), 2);
        assert!(registry.destroy_entity(*entity));
    });
    valid_entities.iter().for_each(|entity| {
        assert_eq!(registry.destroy_entity(*entity), false);
        assert!(registry.get_entity_entry(*entity).is_none());
        assert!(registry.get_entity_entry_mut(*entity).is_none());
    });
}
