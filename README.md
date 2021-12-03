# Shard

Shard is an Archetype-based Entity Component System implemented in Rust.

# Features
- Systems are (currently) implicit.
- Supports no_std environments. (Requires alloc however)
- Up to 14 components per entity.
- Up to 65K archetypes.
- Up to 16.7 million entities.
- Dependency free.
- (Mostly) unit tested.

# Usage

See the examples directory for details, but in short:

1. Implement Component trait for types which you wish to use as components.
2. Create a Registry.
3. Use the Registry to construct entities.
4. Add or remove components from entities.
5. Iterate over entities matching certain subsets of components.

# TODO:
- Query support
- Filter iterators
- Allow replacing components.
- Allow adding/removing multiple components at once.
- Faster archetype lookups (using a graph).
- Component dependencies/exclusions.
