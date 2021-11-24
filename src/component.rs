use crate::component_descriptor::*;
use crate::{
    component_type_id::ComponentTypeId, define_component_descriptor, fnv1a::fnv1a_hash_str_16_xor,
};

pub trait Component: Send + Sync + Sized + 'static {
    const NAME: &'static str;
    const ID: ComponentTypeId = ComponentTypeId::from_u16(fnv1a_hash_str_16_xor(Self::NAME));
    const DESCRIPTOR: ComponentDescriptor = define_component_descriptor!(Self);
}
