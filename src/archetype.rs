use crate::component_descriptor::ComponentDescriptor;

#[derive(Debug, Clone)]
pub(crate) struct ArchetypeEdge {
    adding: u16,
    removing: u16,
}

impl ArchetypeEdge {
    pub(crate) fn new(adding: u16, removing: u16) -> Self {
        Self { adding, removing }
    }

    pub(crate) const INVALID: Self = Self {
        adding: u16::MAX,
        removing: u16::MAX,
    };
}

pub(crate) struct Archetype<const N: usize> {
    components: [ComponentDescriptor; N],
    edges: [ArchetypeEdge; N],
}

impl<const N: usize> Archetype<N> {
    pub(crate) fn new(components: [ComponentDescriptor; N], edges: [ArchetypeEdge; N]) -> Self {
        Self { components, edges }
    }
}
