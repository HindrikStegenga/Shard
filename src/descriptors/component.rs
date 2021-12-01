use crate::descriptors::component_descriptor::*;
use crate::{
    define_component_descriptor, fnv1a::fnv1a_hash_str_16_xor,
};
use crate::descriptors::component_type_id::ComponentTypeId;

/// Implement this trait to use a type as a component in the ECS.
/// Do not override the default implementations for [`Self::ID`] or [`Self::DESCRIPTOR`].
/// Only implement [`Self::NAME`].
/// # Safety:
/// - size_of<Self> must not exceed u16::MAX.
/// - align_of<Self> must not exceed u16::MAX.
pub trait Component: Send + Sync + Sized + 'static {
    /// Human readable program unique name used for calculating a stable type identifier.
    const NAME: &'static str;
    /// Do not implement this manually. (Unless a hash collision occurs).
    const ID: ComponentTypeId = ComponentTypeId::from_u16(fnv1a_hash_str_16_xor(Self::NAME));
    /// A descriptor defining the component type.
    const DESCRIPTOR: ComponentDescriptor = define_component_descriptor!(Self);
}
