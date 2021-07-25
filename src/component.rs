use crate::{component_type_id::ComponentTypeId, fnv1a::fnv1a_hash_str_16_xor};

pub trait Component: Send + Sync + Sized + 'static {
    const NAME: &'static str;
    const ID: ComponentTypeId = ComponentTypeId::from_u16(fnv1a_hash_str_16_xor(Self::NAME));
}
