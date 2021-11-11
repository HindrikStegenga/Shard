use crate::ArchetypeId;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) struct SortedArchetypeKey {
    pub(super) id: ArchetypeId,
    pub(super) archetype_index: u16,
}
