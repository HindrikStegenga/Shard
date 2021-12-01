# Shard

Shard is an Archetype-based Entity Component System implemented in Rust.

# Features
- Supports no_std environments. (Requires alloc however)
- Up to 14 components per entity.
- Up to 65K archetypes.
- Up to 16.7 million entities.
- Dependency free.
- (Mostly) unit tested.

# TODO:
- Query support
- Filter iterators
- Allow replacing components.
- Allow adding/removing multiple components at once.
- Faster archetype lookups.
