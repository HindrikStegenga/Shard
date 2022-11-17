#[cfg(test)]
use crate::test_components::*;
#[cfg(test)]
use crate::*;
use alloc::vec::Vec;

#[test]
fn registry_test_get_component() {
    let mut registry = Registry::default();
    let entity = registry
        .create_entity((A::default(), B::default()))
        .unwrap();
    assert_eq!(registry.get_component(entity), Some(&A::default()));
    assert_eq!(registry.get_component_mut(entity), Some(&mut B::default()));
    assert_eq!(registry.get_component::<C>(entity), None);
    assert_eq!(registry.get_component_mut::<A>(Entity::invalid()), None);
    assert_eq!(
        registry.get_components::<(A, B)>(entity),
        Some((&A::default(), &B::default()))
    );
    assert_eq!(
        registry.get_components::<(B, A)>(entity),
        Some((&B::default(), &A::default()))
    );
    assert_eq!(registry.get_components::<(A, B)>(Entity::invalid()), None);
    assert_eq!(
        registry.get_components_mut::<(A, B)>(entity),
        Some((&mut A::default(), &mut B::default()))
    );
    assert_eq!(
        registry.get_components_mut::<(B, A)>(entity),
        Some((&mut B::default(), &mut A::default()))
    );
    assert_eq!(
        registry.get_components_mut::<(A, B)>(Entity::invalid()),
        None
    );
    assert_eq!(
        registry.get_component::<A>(entity),
        registry.get_components::<A>(entity)
    );
}

#[test]
fn registry_test_has_component() {
    let mut registry: Registry = Default::default();
    let entity = registry
        .create_entity((A::default(), B::default()))
        .unwrap();
    assert_eq!(registry.has_component::<A>(entity), true);
    assert_eq!(registry.has_component::<B>(entity), true);
    assert_eq!(registry.has_component::<C>(entity), false);
    assert_eq!(registry.has_component::<A>(Entity::invalid()), false);

    assert_eq!(registry.has_components::<(A, B)>(entity), true);
    assert_eq!(registry.has_components::<(A, C)>(entity), false);
    assert_eq!(registry.has_components::<(A, B)>(Entity::invalid()), false);
}

#[test]
fn test_registry() {
    let mut registry = Registry::default();
    let entity = registry.create_entity((A::default(), B::default()));
    assert!(entity.is_ok());
    let entity = entity.unwrap();
    assert_eq!(entity.version(), 0);
    let component = registry.get_component::<A>(entity);
    assert!(component.is_some());
    let component = component.unwrap();
    assert_eq!(*component, A::default());
    let component = registry.get_component_mut::<B>(entity);
    assert!(component.is_some());
    let component = component.unwrap();
    assert_eq!(*component, B::default());
    registry.destroy_entity(entity);
    let entity = registry.create_entity((A::default(), B::default()));
    assert!(entity.is_ok());
    let entity = entity.unwrap();
    assert_eq!(entity.version(), 1);
    let component = registry.get_component::<A>(entity);
    assert!(component.is_some());
    let component = component.unwrap();
    assert_eq!(*component, A::default());
    let component = registry.get_component_mut::<B>(entity);
    assert!(component.is_some());
    let component = component.unwrap();
    assert_eq!(*component, B::default());

    let entity_data = registry.remove_entity::<(B, A)>(entity);
    assert!(entity_data.is_some());
    let (b, a) = entity_data.unwrap();
    assert_eq!(a, A::default());
    assert_eq!(b, B::default());

    let entity = registry.create_entity((A::default(), B::default()));
    assert!(entity.is_ok());
    let entity = entity.unwrap();
    assert!(registry.has_component::<B>(entity));
    assert!(registry.has_component::<A>(entity));

    assert!(registry.add_component(entity, C::default()).is_ok());

    assert!(registry.has_component::<B>(entity));
    assert!(registry.has_component::<A>(entity));
    assert!(registry.has_component::<C>(entity));

    assert!(registry.remove_component::<A>(entity).is_ok());

    assert!(registry.has_components::<(C, B)>(entity));
    assert_eq!(registry.has_component::<A>(entity), false);

    registry.destroy_entity(entity);

    const COUNT: usize = 128;
    let mut registry = Registry::default();
    let entities: Vec<_> = (0..COUNT)
        .map(|e| {
            registry
                .create_entity((A { _data: e }, B { _data: COUNT - e }))
                .unwrap()
        })
        .collect();

    registry
        .iter_entities()
        .for_each(|e| assert!(entities.contains(&e)));

    for (entities, (a, b)) in registry.iter_entity_components_matching::<(A, B)>() {
        entities.iter().for_each(|e| assert!(entities.contains(&e)));
        a.iter()
            .enumerate()
            .for_each(|(i, a)| assert_eq!(a._data, i));
        b.iter()
            .enumerate()
            .for_each(|(i, b)| assert_eq!(b._data, COUNT - i));
    }

    let _entities2: Vec<_> = (0..COUNT)
        .map(|e| {
            registry
                .create_entity((A { _data: e }, B { _data: COUNT - e }, C { _data: e }))
                .unwrap()
        })
        .collect();
    let mut counter = 0;
    for _ in registry.iter_components_matching::<(A, B)>() {
        counter += 1;
    }
    assert_eq!(counter, 2);

    let entities = (0..1000)
        .into_iter()
        .map(|_e| {
            registry
                .create_entity((A::default(), B::default()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    for entity in &entities {
        let entity = entity.clone();
        assert!(registry.add_component(entity, C::default()).is_ok());
        assert!(registry.has_component::<C>(entity));
    }
    for entity in &entities {
        let entity = entity.clone();
        match rand::random() {
            true => {
                assert!(registry.destroy_entity(entity))
            }
            false => match rand::random() {
                true => {
                    assert!(registry.remove_component::<C>(entity).is_ok());
                    assert!(registry.has_components::<(A, B)>(entity));
                    assert_eq!(registry.has_component::<C>(entity), false);
                    assert_eq!(
                        registry.get_components::<(A, B)>(entity),
                        Some((&A::default(), &B::default()))
                    );
                }
                false => {
                    assert!(registry.remove_component::<A>(entity).is_ok());
                    assert!(registry.has_components::<(C, B)>(entity));
                    assert_eq!(registry.has_component::<A>(entity), false);
                    assert_eq!(
                        registry.get_components::<(C, B)>(entity),
                        Some((&C::default(), &B::default()))
                    );
                }
            },
        }
    }
}
