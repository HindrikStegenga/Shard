use shard_ecs::*;
#[allow(unused)]

// Some random struct.
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

// Implement the Component trait for it like this.
// You usually should not implement any of it's other associated const's otherwise.
impl Component for Position {
    const NAME: &'static str = "Position";
}

// You can also use a derive macro if the "derive" feature is enabled:
#[cfg(feature = "derive")]
#[derive(Component)]
struct DeriveExample {
    foo: f32,
}

fn main() {
    // code ..
}
