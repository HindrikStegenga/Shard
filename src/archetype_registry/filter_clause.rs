use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::ComponentGroup;

pub trait ComponentFilterGroup: private::SealedFilterClause {
    const EXCLUSIONS: &'static ArchetypeDescriptor;
}

impl ComponentFilterGroup for () {
    const EXCLUSIONS: &'static ArchetypeDescriptor = &ArchetypeDescriptor::INVALID;
}

impl<'a, G:ComponentGroup<'a>> ComponentFilterGroup for G {
    const EXCLUSIONS: &'static ArchetypeDescriptor = G::DESCRIPTOR.archetype();
}

mod private {
    use crate::ComponentGroup;

    pub trait SealedFilterClause {}

    impl SealedFilterClause for () {}
    impl<'a, G: ComponentGroup<'a>> SealedFilterClause for G {}
}