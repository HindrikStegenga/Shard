use shard_ecs::*;
#[allow(unused)]
#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}
impl Component for Position {
    const NAME: &'static str = "Position";
}
#[allow(unused)]
#[derive(Debug, Clone)]
struct Rotation {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
impl Component for Rotation {
    const NAME: &'static str = "Position";
}

#[allow(unused)]
fn main() {
    let position = Position {
        x: 2.0,
        y: 9.1,
        z: -2.0,
    };
    let rotation = Rotation {
        x: 3.0,
        y: 5.0,
        z: 1.0,
        w: 7.0,
    };

    let mut registry = Registry::default();
    // Entities cannot exist without them having components.
    // As such, you create an entity by giving it an initial set of components.
    let result = registry.create_entity(position.clone());
    // Keep in mind, it returns a Result<Entity,G>, as it could fail, returning the component instance.
    let entity = result.unwrap();

    // We can add a component to the entity:
    let result = registry.add_component(entity, rotation);
    // In case of failure, for example the entity already has a component of the specific type, an Err(component) is returned.

    // The can also be removed. This will remove the component returning it if successful.
    // If the entity doesn't have the specified component, it returns an Err().
    let result = registry.remove_component::<Position>(entity);
}
