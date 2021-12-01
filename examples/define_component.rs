use shard_ecs::*;

// Some random struct.
struct Position {
    x: f32, y: f32, z: f32
}

// Implement the Component trait for it like this.
// You usually should not implement any of it's other associated const's otherwise.
impl Component for Position {
    const NAME: &'static str = "Position";
}


fn main() {
    // code ..
}