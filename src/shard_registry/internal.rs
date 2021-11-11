use super::*;

impl Default for ShardRegistry {
    fn default() -> Self {
        Self {
            shards: Vec::with_capacity(256),
            archetypes: [
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
            ],
            sorted_mappings: [
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(512),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(256),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(128),
                Vec::with_capacity(64),
                Vec::with_capacity(64),
            ],
            next_recyclable_shard: None,
        }
    }
}

impl ShardRegistry {
    fn drop_shard(archetype: &mut Archetype, shard: &mut Shard) {}

    #[cfg(debug_assertions)]
    fn verify_consistency(&self) -> bool {
        self.archetypes.iter().for_each(|e| {
            e.iter().for_each(|e| {
                assert_ne!(e.first_shard_index(), 0);
            });
        });
        return true;
    }
}
