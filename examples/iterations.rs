use shard_ecs::*;
#[allow(unused)]
#[derive(Clone)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}
impl Component for Position {
    const NAME: &'static str = "Position";
}

#[allow(unused)]
#[derive(Clone)]
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

    (0..10000).into_iter().for_each(|_| {
        // Add a bunch of entities to the registry.
        registry.create_entity((position.clone(), rotation.clone()));
    });

    // Iteration over all entity handles in the system is possible:
    registry.iter_entities().for_each(|entity| {
        // Do stuff here with the entity handle.
    });

    // Iteration for entities exclusively matching a set of components:
    let (position_slice, rotation_slice) = registry.iter_components_exact::<(Position, Rotation)>();
    // This will match entities which have exactly the specified set of components.

    // Iterate over the slices.
    position_slice
        .iter()
        .zip(rotation_slice)
        .for_each(|(rotation, position)| {
            // Do something with the rotation nand iteration.
        });

    // You can also match all entities with specified components. This means that it will also
    // entities which contain more than the specified components.
    registry
        .iter_components_matching::<Position>()
        .for_each(|position_slice| {
            // For example, the above match will match entities with (Position) but also with (Position, Rotation).
            position_slice.iter().for_each(|position| {
                // Do something with the position object, like compute a new position it.
            })
        });

    // Arbitrary filtering is also supported. As an example, archetypes including a specific component can be excluded.
    registry
        .iter_filtered_components_matching::<Position, _>(|a| {
            !a.has_component::<Rotation>() // All archetypes with a position match, except those which also have a rotation.
                                           // This closure must return true for everything that needs to be included, false otherwise.
        })
        .for_each(|position_slice| {
            // For example, the above match will match entities with (Position) but also with (Position, Rotation).
            position_slice.iter().for_each(|position| {
                // Do something with the position object, like compute a new position it.
            })
        });

    // You can also iterate over component slices with their associated entity handles:
    registry
        .iter_entity_components_matching::<Rotation>()
        .for_each(|(entities_slice, rotation_slice)| {
            entities_slice
                .iter()
                .zip(rotation_slice)
                .for_each(|(entity, rotation)| {
                    // You have access to the entity handle here as well.
                })
        })
}
