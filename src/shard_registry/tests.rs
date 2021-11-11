use crate::component_group::ComponentGroup;
use crate::component_group_descriptor::ComponentGroupDescriptor;
use crate::shard_registry::ShardRegistry;
use crate::tests::*;

#[test]
fn test_create_default_shard_registry() {
    ShardRegistry::default();
}

#[test]
fn test_shard_registry() {
    let mut registry = ShardRegistry::default();

    let shard = registry.find_or_create_single_entity_shard_from_group::<(A, B)>();
    assert!(shard.is_some());
    let shard = shard.unwrap();
}
