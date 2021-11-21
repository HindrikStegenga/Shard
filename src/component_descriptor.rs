use core::mem::ManuallyDrop;

use crate::{
    archetype_descriptor::ArchetypeDescriptor, component_type_id::ComponentTypeId, Component,
};

#[macro_export]
macro_rules! define_component_descriptor {
    ($item:ident) => {
        ComponentDescriptor {
            component_type_id: $item::ID,
            size: core::mem::size_of::<$item>() as u16,
            align: core::mem::align_of::<$item>() as u16,
            fns: ComponentDescriptorFnPointers {
                drop_handler: ComponentDescriptor::drop_handler_wrapper::<$item>,
            },
        }
    };
}

#[macro_export]
macro_rules! copy_component_descriptor_from_to {
    ($source:expr, $destination:expr) => {
        $destination.component_type_id = $source.component_type_id;
        $destination.size = $source.size;
        $destination.align = $source.align;
        $destination.fns = $source.fns;
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComponentDescriptorFnPointers {
    pub(crate) drop_handler: unsafe fn(ptr: *mut u8, len: usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentDescriptor {
    pub(crate) component_type_id: ComponentTypeId,
    pub(crate) size: u16,
    pub(crate) align: u16,
    pub(crate) fns: ComponentDescriptorFnPointers,
}

impl Into<ArchetypeDescriptor> for &ComponentDescriptor {
    fn into(self) -> ArchetypeDescriptor {
        ArchetypeDescriptor::new(
            self.component_type_id.into(),
            1,
            [
                self.clone(),
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
            ],
        )
    }
}

impl Into<ArchetypeDescriptor> for ComponentDescriptor {
    fn into(self) -> ArchetypeDescriptor {
        ArchetypeDescriptor::new(
            self.component_type_id.into(),
            1,
            [
                self,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
                ComponentDescriptor::INVALID,
            ],
        )
    }
}

impl ComponentDescriptor {
    pub const INVALID: Self = {
        unsafe fn _dummy_drop_(_ptr: *mut u8, _len: usize) {}
        ComponentDescriptor {
            component_type_id: ComponentTypeId::INVALID,
            size: 0,
            align: 0,
            fns: ComponentDescriptorFnPointers {
                drop_handler: _dummy_drop_,
            },
        }
    };

    pub fn from_component<C: Component>() -> ComponentDescriptor {
        Self::new(
            C::ID,
            core::mem::size_of::<C>() as u16,
            core::mem::align_of::<C>() as u16,
            Self::drop_handler_wrapper::<C>,
        )
    }

    pub fn new(
        component_type_id: ComponentTypeId,
        size: u16,
        align: u16,
        drop_handler: unsafe fn(ptr: *mut u8, len: usize),
    ) -> Self {
        if !component_type_id.is_valid() {
            return Self::INVALID;
        }

        Self {
            component_type_id,
            size,
            align,
            fns: ComponentDescriptorFnPointers { drop_handler },
        }
    }

    pub unsafe fn drop_handler_wrapper<C: Component>(ptr: *mut u8, len: usize) {
        let s = core::slice::from_raw_parts_mut(ptr as *mut ManuallyDrop<C>, len);
        s.iter_mut().for_each(|e| ManuallyDrop::drop(e))
    }

    /// Get a the component descriptor's component type id.
    pub const fn component_type_id(&self) -> ComponentTypeId {
        self.component_type_id
    }

    /// Get a the component descriptor's size.
    pub const fn size(&self) -> u16 {
        self.size
    }

    /// Get a the component descriptor's align.
    pub const fn align(&self) -> u16 {
        self.align
    }
}
