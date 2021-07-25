use crate::{component_type_id::ComponentTypeId, MAX_COMPONENTS_PER_ENTITY};

#[derive(Debug, Clone)]
pub struct ComponentDescriptor {
    component_type_id: ComponentTypeId,
    size: usize,
    align: usize,
}

impl ComponentDescriptor {
    pub(crate) const INVALID: Self = ComponentDescriptor {
        component_type_id: ComponentTypeId::INVALID,
        size: 0,
        align: 0,
    };

    pub const fn new(component_type_id: ComponentTypeId, size: usize, align: usize) -> Self {
        if !component_type_id.is_valid() {
            return Self::INVALID;
        }

        Self {
            component_type_id,
            size,
            align,
        }
    }

    /// Get a the component descriptor's component type id.
    pub const fn component_type_id(&self) -> ComponentTypeId {
        self.component_type_id
    }

    /// Get a the component descriptor's size.
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Get a the component descriptor's align.
    pub const fn align(&self) -> usize {
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

    /// Writes the slice into a fixed size array.
    /// # Safety:
    /// - Slice must be <= MAX_COMPONENTS_PER_ENTITY
    pub const unsafe fn write_into_fixed_size_array(
        descriptors: &[ComponentDescriptor],
    ) -> [ComponentDescriptor; MAX_COMPONENTS_PER_ENTITY] {
        let mut temp = [ComponentDescriptor::INVALID; MAX_COMPONENTS_PER_ENTITY];
        let mut i = 0;
        while i < descriptors.len() {
            temp[i].size = descriptors[i].size;
            temp[i].align = descriptors[i].align;
            temp[i].component_type_id = descriptors[i].component_type_id;
            i += 1;
        }
        temp
    }

    /// Computes sorted descriptor array. Assumes that the passed array is valid.
    pub(crate) const fn compute_sorted_descriptors<const N: usize>(
        descriptors: [ComponentDescriptor; N],
    ) -> [ComponentDescriptor; N] {
        let mut sorted_descriptors: [ComponentDescriptor; N] = {
            let mut temp = [ComponentDescriptor::INVALID; N];
            let mut i = 0;
            while i < descriptors.len() {
                temp[i].size = descriptors[i].size;
                temp[i].align = descriptors[i].align;
                temp[i].component_type_id = descriptors[i].component_type_id;
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
                    let tmp = ComponentDescriptor::new(
                        sorted_descriptors[i].component_type_id,
                        sorted_descriptors[i].size,
                        sorted_descriptors[i].align,
                    );
                    sorted_descriptors[i].size = sorted_descriptors[j].size;
                    sorted_descriptors[i].align = sorted_descriptors[j].align;
                    sorted_descriptors[i].component_type_id =
                        sorted_descriptors[j].component_type_id;
                    sorted_descriptors[j] = tmp;
                }
                j += 1;
            }
            i += 1;
        }
        descriptors
    }
}
