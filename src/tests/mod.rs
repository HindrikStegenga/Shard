#[cfg(test)]
extern crate std;

use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_descriptor::ComponentDescriptor;

mod component_group;
mod entity;

mod entity_registry;

mod shard_registry;

#[test]
fn test() {
    std::println!("{}", core::mem::size_of::<ComponentDescriptor>());
    std::println!("{}", core::mem::size_of::<ArchetypeDescriptor>());
}
