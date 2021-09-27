#[cfg(test)]
extern crate std;

use crate::archetype::*;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_descriptor::ComponentDescriptor;
use crate::entity_registry::*;
use crate::shard::Shard;

mod component_group;
mod entity;

mod entity_registry;

mod shard_registry;

#[test]
fn test() {
    std::println!(
        "EntityEntryState: {}",
        core::mem::align_of::<EntityEntryState>()
    );
    std::println!("EntityEntry: {}", core::mem::align_of::<EntityEntry>());

    std::println!(
        "EntityEntryState: {}",
        core::mem::size_of::<EntityEntryState>()
    );
    std::println!("EntityEntry: {}", core::mem::size_of::<EntityEntry>());
    std::println!(
        "ComponentDescriptor: {}",
        core::mem::size_of::<ComponentDescriptor>()
    );
    std::println!(
        "ArchetypeDescriptor: {}",
        core::mem::size_of::<ArchetypeDescriptor>()
    );
    std::println!("Archetype: {}", core::mem::size_of::<Archetype>());
    std::println!("Shard: {}", core::mem::size_of::<Shard>());
}
