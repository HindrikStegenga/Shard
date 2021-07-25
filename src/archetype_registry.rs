use alloc::vec::Vec;

use crate::{
    archetype_descriptor::ArchetypeDescriptor, component_descriptor::ComponentDescriptor,
    ComponentTypeId, MAX_COMPONENTS_PER_ENTITY,
};

struct ComponentEntry {
    id: ComponentTypeId,
    size: usize,
    align: usize,
    drop_handler: fn(*mut u8),
}

pub(crate) struct ArchetypeEntry<const N: usize> {
    sorted_components: [ComponentEntry; N],
    shards: Vec<u16>,
    super_archetypes: Vec<u16>,
    sub_archetypes: Vec<u16>,
}

pub(crate) struct ArchetypeRegistry {
    archetypes_1: Vec<ArchetypeEntry<1>>,
    archetypes_2: Vec<ArchetypeEntry<2>>,
    archetypes_3: Vec<ArchetypeEntry<3>>,
    archetypes_4: Vec<ArchetypeEntry<4>>,

    archetypes_5: Vec<ArchetypeEntry<5>>,
    archetypes_6: Vec<ArchetypeEntry<6>>,
    archetypes_7: Vec<ArchetypeEntry<7>>,
    archetypes_8: Vec<ArchetypeEntry<8>>,

    archetypes_9: Vec<ArchetypeEntry<9>>,
    archetypes_10: Vec<ArchetypeEntry<10>>,
    archetypes_11: Vec<ArchetypeEntry<11>>,
    archetypes_12: Vec<ArchetypeEntry<12>>,

    archetypes_13: Vec<ArchetypeEntry<13>>,
    archetypes_14: Vec<ArchetypeEntry<14>>,
    archetypes_15: Vec<ArchetypeEntry<15>>,
    archetypes_16: Vec<ArchetypeEntry<16>>,
}

impl Default for ArchetypeRegistry {
    fn default() -> Self {
        Self {
            archetypes_1: Vec::with_capacity(128),
            archetypes_2: Vec::with_capacity(128),
            archetypes_3: Vec::with_capacity(128),
            archetypes_4: Vec::with_capacity(128),
            archetypes_5: Vec::with_capacity(64),
            archetypes_6: Vec::with_capacity(64),
            archetypes_7: Vec::with_capacity(64),
            archetypes_8: Vec::with_capacity(64),
            archetypes_9: Vec::with_capacity(32),
            archetypes_10: Vec::with_capacity(32),
            archetypes_11: Vec::with_capacity(32),
            archetypes_12: Vec::with_capacity(32),
            archetypes_13: Vec::with_capacity(16),
            archetypes_14: Vec::with_capacity(16),
            archetypes_15: Vec::with_capacity(16),
            archetypes_16: Vec::with_capacity(16),
        }
    }
}

impl ArchetypeRegistry {}
