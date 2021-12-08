use crate::ArchetypeId;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(in crate::archetype_registry) struct SortedArchetypeKey {
    pub(crate) id: ArchetypeId,
    pub(in crate::archetype_registry) archetype_index: u16,
}
