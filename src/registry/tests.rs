#[cfg(test)]
use crate::test_components::*;
#[cfg(test)]
use crate::*;
use alloc::vec::Vec;

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

    registry.destroy_entity(entity);

    const COUNT: usize = 1024;
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
}
