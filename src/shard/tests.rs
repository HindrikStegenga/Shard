use super::*;
use crate::tests::*;

#[test]
fn test_shard() {
    unsafe {
        let group = (A::default(), B::default(), C::default());
        let descriptor = <(A, B, C) as ComponentGroup<'_>>::DESCRIPTOR.archetype();

        let mut shard = Shard::new(&descriptor, 0);
        assert!(shard.is_some());
        let mut shard = shard.unwrap();

        let meta = EntityMetadata::default();

        let idx = shard.push_entity_unchecked(meta, (A::default(), B::default(), C::default()));
        assert_eq!(shard.entity_count, 1);
        assert_eq!(meta, shard.entities.as_ref()[0]);

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

        let slices: (&[B], &[A]) = shard.get_fuzzy_slices_unchecked::<(B, A)>(descriptor);
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], B::default());
        assert_eq!(slices.1[0], A::default());

        let slices: (&[A], &[C]) = shard.get_fuzzy_slices_unchecked::<(A, C)>(descriptor);
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], A::default());
        assert_eq!(slices.1[0], C::default());
    }
}
