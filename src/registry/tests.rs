#[test]
fn test_registry() {
    use crate::test_components::*;
    use crate::*;

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
}
