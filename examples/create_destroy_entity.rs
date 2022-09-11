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
    // Use entity for various things.

    // Multiple components can be added to the entity by using a tuple of them.
    // Order is irrelevant, the result is the same.
    let result = registry.create_entity((position, rotation));

    // Entities can also be destroyed, returning true if the entity was destroyed.
    let result = registry.destroy_entity(entity);
    // The entity handle is now free for re-use. The old entity handle is now invalid.
    // This means it cannot be used anymore and doing so will fail any operation on the registry.
    // Therefore, you can freely copy around the entity handle to refer to an entity and when it gets destroyed,
    // the handle will not refer to a valid entity anymore and operations on the registry will thus fail.
    // Internally, each entity handle contains a version segment which rolls over until a specific
    // internal entity slot is re-used 256 times. Under normal circumstances you won't run into issues with this.
}
