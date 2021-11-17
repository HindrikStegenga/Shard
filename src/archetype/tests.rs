use crate::archetype::Archetype;
use crate::archetype::metadata::EntityMetadata;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_group::ComponentGroup;
use crate::MAX_ENTITIES_PER_ARCHETYPE;
use crate::tests::*;


#[test]
fn test_archetype_constructors() {
    let descriptor : &ArchetypeDescriptor = <(A, B) as ComponentGroup>::DESCRIPTOR.archetype();
    // Empty constructor should not panic
    let _ = Archetype::new(descriptor);
    Archetype::with_capacity(descriptor, 0);
    Archetype::with_capacity(descriptor, MAX_ENTITIES_PER_ARCHETYPE);
    Archetype::with_capacity(descriptor, MAX_ENTITIES_PER_ARCHETYPE + 1);
}

#[test]
fn test_archetype_slices() {
    unsafe {
        let descriptor = <(A, B, C) as ComponentGroup<'_>>::DESCRIPTOR.archetype();

        let mut shard = Archetype::new(descriptor);
        let meta = EntityMetadata::default();

        let idx = shard.push_entity_unchecked(meta, (A::default(), B::default(), C::default()));
        assert_eq!(shard.entity_count, 1);
        assert_eq!(meta, shard.entity_metadata().as_ref()[0]);

        let slices: (&[A], &[B], &[C]) = shard.get_slices_unchecked_exact::<(A, B, C)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], A::default());
        assert_eq!(slices.1[0], B::default());
        assert_eq!(slices.2[0], C::default());
        let slices: (&[B], &[A], &[C]) = shard.get_slices_unchecked_exact::<(B, A, C)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], B::default());
        assert_eq!(slices.1[0], A::default());
        assert_eq!(slices.2[0], C::default());

        let slices: (&[B], &[A]) = shard.get_fuzzy_slices_unchecked::<(B, A)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], B::default());
        assert_eq!(slices.1[0], A::default());

        let slices: (&[A], &[C]) = shard.get_fuzzy_slices_unchecked::<(A, C)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], A::default());
        assert_eq!(slices.1[0], C::default());
    }
}