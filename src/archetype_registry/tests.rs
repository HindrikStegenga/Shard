use crate::archetype_registry::ArchetypeRegistry;
use crate::component_group::ComponentGroup;
use crate::test_components::*;

#[test]
fn test_archetype_registry() {
    let mut registry = ArchetypeRegistry::default();
    let descriptor = <(A, B) as ComponentGroup>::DESCRIPTOR.archetype();
    let archetype = registry.find_or_create_archetype(&descriptor);
}
