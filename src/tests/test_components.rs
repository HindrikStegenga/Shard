use crate::*;

#[derive(Copy, Clone)]
pub(crate) struct A {
    _data: usize,
}
impl Default for A {
    fn default() -> Self {
        Self { _data: 1 }
    }
}
impl Component for A {
    const NAME: &'static str = "A";
    const ID: ComponentTypeId = ComponentTypeId::from_u16(1);
}
#[derive(Copy, Clone)]
pub(crate) struct B {
    _data: usize,
}
impl Default for B {
    fn default() -> Self {
        Self { _data: 2 }
    }
}
impl Component for B {
    const NAME: &'static str = "B";
    const ID: ComponentTypeId = ComponentTypeId::from_u16(2);
}
#[derive(Copy, Clone)]
pub(crate) struct C {
    _data: usize,
}
impl Default for C {
    fn default() -> Self {
        Self { _data: 3 }
    }
}
impl Component for C {
    const NAME: &'static str = "C";
    const ID: ComponentTypeId = ComponentTypeId::from_u16(3);
}
