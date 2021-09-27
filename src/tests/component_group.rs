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
    fn test_group_len<C: ComponentGroup>(expected_len: usize) {
        assert_eq!(C::LENGTH as usize, expected_len);
    }

    test_group_len::<Position>(1);
    test_group_len::<(Position, Rotation)>(2);
    test_group_len::<(Position, Rotation, Velocity)>(3);

    test_group_len::<(Position, Rotation, Position)>(0);
}
