use core::mem::ManuallyDrop;

use crate::{
    archetype_descriptor::ArchetypeDescriptor, component_type_id::ComponentTypeId, Component,
    MAX_COMPONENTS_PER_ENTITY,
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
macro_rules! copy_from_component_descriptor {
    ($destination:expr, $source:expr) => {
        $destination.component_type_id = $source.component_type_id;
        $destination.size = $source.size;
        $destination.align = $source.align;
        $destination.fns = $source.fns;
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ComponentDescriptorFnPointers {
    pub(crate) drop_handler: unsafe fn(ptr: *mut u8, len: usize),
}

#[derive(Debug, Clone)]
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
            fns: ComponentDescriptorFnPointers {
                drop_handler: drop_handler,
            },
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

    /// Validates component descriptor slice. Checks length, duplicates and sorting order if desired.
    /// Returns the provided null slice in case of failure.
    pub(crate) const fn validate_descriptors<'a, 'b: 'a>(
        descriptors: &'a [ComponentDescriptor],
        check_sorting: bool,
        null_slice: &'b [ComponentDescriptor],
    ) -> &'a [ComponentDescriptor] {
        if descriptors.len() > MAX_COMPONENTS_PER_ENTITY {
            return null_slice;
        }

        let mut idx = 0;
        while idx < descriptors.len() {
            let mut cdx = idx + 1;
            while cdx < descriptors.len() {
                if descriptors[idx].component_type_id().into_u16()
                    == descriptors[cdx].component_type_id().into_u16()
                {
                    return null_slice;
                }
                cdx += 1;
            }
            idx += 1;
        }
        if check_sorting {
            idx = 1;
            while idx < descriptors.len() {
                if descriptors[idx].component_type_id().into_u16()
                    > descriptors[idx - 1].component_type_id().into_u16()
                {
                    return null_slice;
                }
                idx += 1;
            }
        }
        return descriptors;
    }

    /// Validates component descriptor slice. Checks length, duplicates and sorting order if desired.
    /// Returns the provided null slice in case of failure.
    pub(crate) const fn validate_descriptors_fixed<'a, 'b: 'a, const N: usize>(
        descriptors: &'a [ComponentDescriptor; N],
        check_sorting: bool,
    ) -> &'a [ComponentDescriptor; N] {
        if descriptors.len() > MAX_COMPONENTS_PER_ENTITY {
            return &[ComponentDescriptor::INVALID; N];
        }

        let mut idx = 0;
        while idx < descriptors.len() {
            let mut cdx = idx + 1;
            while cdx < descriptors.len() {
                if descriptors[idx].component_type_id().into_u16()
                    == descriptors[cdx].component_type_id().into_u16()
                {
                    return &[ComponentDescriptor::INVALID; N];
                }
                cdx += 1;
            }
            idx += 1;
        }
        if check_sorting {
            idx = 1;
            while idx < descriptors.len() {
                if descriptors[idx].component_type_id().into_u16()
                    > descriptors[idx - 1].component_type_id().into_u16()
                {
                    return &[ComponentDescriptor::INVALID; N];
                }
                idx += 1;
            }
        }
        return descriptors;
    }

    /// Writes the slice into a fixed size array.
    /// # Safety:
    /// - Slice must be <= MAX_COMPONENTS_PER_ENTITY
    pub const unsafe fn write_into_fixed_size_array(
        descriptors: &[ComponentDescriptor],
    ) -> [ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY] {
        let mut temp = [ComponentDescriptor::INVALID; MAX_COMPONENTS_PER_ENTITY];
        let mut i = 0;
        while i < descriptors.len() {
            copy_from_component_descriptor!(temp[i], descriptors[i]);
            i += 1;
        }
        temp
    }

    /// Computes sorted descriptor array. Assumes that the passed array is valid.
    pub(crate) const fn compute_sorted_descriptors<const N: usize>(
        descriptors: &[ComponentDescriptor; N],
    ) -> [ComponentDescriptor; N] {
        let mut sorted_descriptors: [ComponentDescriptor; N] = {
            let mut temp = [ComponentDescriptor::INVALID; N];
            let mut i = 0;
            while i < descriptors.len() {
                copy_from_component_descriptor!(temp[i], descriptors[i]);
                i += 1;
            }
            temp
        };
        let mut i = 0;
        while i < sorted_descriptors.len() {
            let mut j = 0;
            while j < sorted_descriptors.len() {
                //TODO: Whenever const_fn traits are finished, implement a const fn version of Ord
                #[allow(clippy::manual_swap)]
                if sorted_descriptors[j].component_type_id().into_u16()
                    > sorted_descriptors[i].component_type_id().into_u16()
                {
                    let tmp = ComponentDescriptor {
                        component_type_id: sorted_descriptors[i].component_type_id,
                        size: sorted_descriptors[i].size,
                        align: sorted_descriptors[i].align,
                        fns: sorted_descriptors[i].fns,
                    };
                    copy_from_component_descriptor!(sorted_descriptors[i], sorted_descriptors[j]);
                    sorted_descriptors[j] = tmp;
                }
                j += 1;
            }
            i += 1;
        }
        sorted_descriptors
    }
}
