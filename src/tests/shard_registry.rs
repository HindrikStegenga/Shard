use crate::component_descriptor::ComponentDescriptor;
use crate::shard_registry::*;
use crate::*;
struct Position;
struct Rotation;
struct Velocity;
impl Component for Position {
    const NAME: &'static str = "Position";
}
impl Component for Rotation {
    const NAME: &'static str = "Rotation";
}
impl Component for Velocity {
    const NAME: &'static str = "Velocity";
}

#[test]
fn test_shard_registry() {
    // let mut registry: ShardRegistry = Default::default();
    // for _ in 0..10 {
    //     let (_, _) = registry
    //         .get_or_alloc_shard_from_component_descriptor(&ComponentDescriptor::from_component::<
    //             Position,
    //         >())
    //         .unwrap();
    //     let (_, _) = registry
    //         .get_or_alloc_shard_from_component_descriptor(&ComponentDescriptor::from_component::<
    //             Rotation,
    //         >())
    //         .unwrap();
    // }
}
