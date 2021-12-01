use crate::archetype_registry::ArchetypeRegistry;
use crate::descriptors::component_group::ComponentGroup;
use crate::entity_registry::entity::*;
use crate::test_components::*;

#[test]
fn test_archetype_registry() {
    let mut registry = ArchetypeRegistry::default();
    let descriptor = <(A, B) as ComponentGroup>::DESCRIPTOR.archetype();
    let (_, archetype) = registry.find_or_create_archetype(&descriptor).unwrap();
    (0..1048576).for_each(|e| unsafe {
        let _ = archetype.push_entity_unchecked(
            Entity::new_unchecked(e as u32, 0),
            (A { _data: e }, B { _data: 1048576 - e }),
        );
    });

    for v in registry.iter_components_matching::<A>() {
        for (idx, elem) in v.iter().enumerate() {
            assert_eq!(elem._data, idx)
        }
    }
    for (a, b) in registry.iter_components_matching::<(A, B)>() {
        for (idx, elem) in a.iter().enumerate() {
            assert_eq!(elem._data, idx)
        }
        for (idx, elem) in b.iter().enumerate() {
            assert_eq!(elem._data, 1048576 - idx)
        }
    }

    for v in registry.iter_components_matching_mut::<A>() {
        for (idx, elem) in v.iter().enumerate() {
            assert_eq!(elem._data, idx)
        }
    }
    for (a, b) in registry.iter_components_matching_mut::<(A, B)>() {
        for (idx, elem) in a.iter().enumerate() {
            assert_eq!(elem._data, idx)
        }
        for (idx, elem) in b.iter().enumerate() {
            assert_eq!(elem._data, 1048576 - idx)
        }
    }
}
