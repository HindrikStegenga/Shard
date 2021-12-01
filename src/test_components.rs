use crate::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct A {
    pub _data: usize,
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
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct B {
    pub _data: usize,
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
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct C {
    pub _data: usize,
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
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DropLogA {
    pub _data: usize,
}
impl Default for DropLogA {
    fn default() -> Self {
        Self { _data: 4 }
    }
}
impl Component for DropLogA {
    const NAME: &'static str = "DropLogA";
    const ID: ComponentTypeId = ComponentTypeId::from_u16(4);
}
#[cfg(test)]
impl Drop for DropLogA {
    fn drop(&mut self) {
        extern crate std;
        std::println!("Dropping A: {:#?}", self as *const Self);
    }
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DropLogB {
    pub _data: usize,
}
impl Default for DropLogB {
    fn default() -> Self {
        Self { _data: 5 }
    }
}
impl Component for DropLogB {
    const NAME: &'static str = "DropLogB";
    const ID: ComponentTypeId = ComponentTypeId::from_u16(5);
}
#[cfg(test)]
impl Drop for DropLogB {
    fn drop(&mut self) {
        extern crate std;
        std::println!("Dropping B: {:#?}", self as *const Self);
    }
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DropLogC {
    pub _data: usize,
}
impl Default for DropLogC {
    fn default() -> Self {
        Self { _data: 6 }
    }
}
impl Component for DropLogC {
    const NAME: &'static str = "DropLogC";
    const ID: ComponentTypeId = ComponentTypeId::from_u16(6);
}
#[cfg(test)]
impl Drop for DropLogC {
    fn drop(&mut self) {
        extern crate std;
        std::println!("Dropping C: {:#?}", self as *const Self);
    }
}
