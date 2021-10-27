use crate::{component_group::ComponentGroup, Component};

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
fn test_component_group_len() {
    fn test_group_len<'c, G: ComponentGroup<'c>>(expected_len: usize) {
        assert_eq!(
            G::DESCRIPTOR.unwrap().archetype().len() as usize,
            expected_len
        );
    }

    test_group_len::<Position>(1);
    test_group_len::<(Position, Rotation)>(2);
    test_group_len::<(Position, Rotation, Velocity)>(3);

    test_group_len::<(Position, Rotation, Position)>(0);
}

#[test]
fn test_component_group_descriptor() {
    #[cfg(test)]
    extern crate std;

    assert!(<Position as ComponentGroup>::DESCRIPTOR.is_some());
    assert!(<(Position, Position) as ComponentGroup>::DESCRIPTOR.is_none());
    assert!(<(Position, Rotation) as ComponentGroup>::DESCRIPTOR.is_some());
    std::println!(
        "{:#?}",
        <(Position, Rotation) as ComponentGroup>::DESCRIPTOR
    );
}
