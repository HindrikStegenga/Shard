use crate::archetype::metadata::EntityMetadata;
use crate::archetype::Archetype;
use crate::archetype_descriptor::ArchetypeDescriptor;
use crate::component_group::ComponentGroup;
use crate::test_components::*;
use crate::{Entity, MAX_ENTITIES_PER_ARCHETYPE};

#[test]
fn test_archetype_constructors() {
    let descriptor: &ArchetypeDescriptor = <(A, B) as ComponentGroup>::DESCRIPTOR.archetype();
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

        let mut archetype = Archetype::new(descriptor);
        let meta = EntityMetadata::default();

        let _idx =
            archetype.push_entity_unchecked(meta, (A::default(), B::default(), C::default()));
        assert_eq!(archetype.entity_count, 1);
        assert_eq!(meta, archetype.entity_metadata().as_ref()[0]);

        let slices: (&[A], &[B], &[C]) = archetype.get_slices_unchecked_exact::<(A, B, C)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], A::default());
        assert_eq!(slices.1[0], B::default());
        assert_eq!(slices.2[0], C::default());
        let slices: (&[B], &[A], &[C]) = archetype.get_slices_unchecked_exact::<(B, A, C)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], B::default());
        assert_eq!(slices.1[0], A::default());
        assert_eq!(slices.2[0], C::default());

        let slices: (&[B], &[A]) = archetype.get_fuzzy_slices_unchecked::<(B, A)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], B::default());
        assert_eq!(slices.1[0], A::default());

        let slices: (&[A], &[C]) = archetype.get_fuzzy_slices_unchecked::<(A, C)>();
        assert_eq!(slices.0.len(), 1);
        assert_eq!(slices.1.len(), 1);
        assert_eq!(slices.0[0], A::default());
        assert_eq!(slices.1[0], C::default());
    }
}

#[test]
fn test_archetype_get_components() {
    unsafe {
        let descriptor = <(A, B, C) as ComponentGroup<'_>>::DESCRIPTOR.archetype();

        let mut archetype = Archetype::new(descriptor);
        let meta = EntityMetadata::default();

        let idx = archetype.push_entity_unchecked(meta, (A::default(), B::default(), C::default()));
        assert_eq!(*archetype.get_component_unchecked::<A>(idx), A::default());
        assert_eq!(
            *archetype.get_component_unchecked_mut::<B>(idx),
            B::default()
        )
    }
}

#[test]
fn test_archetype_swap_entities() {
    unsafe {
        let descriptor = <(A, B) as ComponentGroup<'_>>::DESCRIPTOR.archetype();

        let mut archetype = Archetype::new(descriptor);
        let meta1 = EntityMetadata::new(Entity::from_raw(1));
        let meta2 = EntityMetadata::new(Entity::from_raw(2));

        let idx1 = archetype.push_entity_unchecked(meta1, (A { _data: 1 }, B { _data: 3 }));
        let idx2 = archetype.push_entity_unchecked(meta2, (A { _data: 2 }, B { _data: 4 }));
        assert_eq!(
            *archetype.get_component_unchecked::<A>(idx1),
            A { _data: 1 }
        );
        assert_eq!(
            archetype.read_component_unchecked::<A>(idx1),
            A { _data: 1 }
        );
        assert_eq!(
            *archetype.get_component_unchecked_mut::<B>(idx1),
            B { _data: 3 }
        );
        assert_eq!(
            archetype.read_component_unchecked::<B>(idx1),
            B { _data: 3 }
        );
        assert_eq!(
            *archetype.get_component_unchecked::<A>(idx2),
            A { _data: 2 }
        );
        assert_eq!(
            archetype.read_component_unchecked::<A>(idx2),
            A { _data: 2 }
        );
        assert_eq!(
            *archetype.get_component_unchecked_mut::<B>(idx2),
            B { _data: 4 }
        );
        assert_eq!(
            archetype.read_component_unchecked::<B>(idx2),
            B { _data: 4 }
        );

        assert_eq!(archetype.entity_metadata()[idx1 as usize], meta1);
        assert_eq!(archetype.entity_metadata()[idx2 as usize], meta2);

        archetype.swap_entities(idx1, idx2);
        assert_eq!(
            *archetype.get_component_unchecked::<A>(idx2),
            A { _data: 1 }
        );
        assert_eq!(
            *archetype.get_component_unchecked_mut::<B>(idx2),
            B { _data: 3 }
        );
        assert_eq!(
            *archetype.get_component_unchecked::<A>(idx1),
            A { _data: 2 }
        );
        assert_eq!(
            *archetype.get_component_unchecked_mut::<B>(idx1),
            B { _data: 4 }
        );

        assert_eq!(archetype.entity_metadata()[idx2 as usize], meta1);
        assert_eq!(archetype.entity_metadata()[idx1 as usize], meta2);

        archetype.swap_entities(idx1, idx2);
        assert_eq!(
            *archetype.get_component_unchecked::<A>(idx1),
            A { _data: 1 }
        );
        assert_eq!(
            *archetype.get_component_unchecked_mut::<B>(idx1),
            B { _data: 3 }
        );
        assert_eq!(
            *archetype.get_component_unchecked::<A>(idx2),
            A { _data: 2 }
        );
        assert_eq!(
            *archetype.get_component_unchecked_mut::<B>(idx2),
            B { _data: 4 }
        );

        assert_eq!(archetype.entity_metadata()[idx1 as usize], meta1);
        assert_eq!(archetype.entity_metadata()[idx2 as usize], meta2);
    }
}

#[test]
fn test_archetype_read_components() {
    unsafe {
        let descriptor = <(A, B) as ComponentGroup<'_>>::DESCRIPTOR.archetype();

        let mut archetype = Archetype::new(descriptor);
        let meta = EntityMetadata::default();

        let idx = archetype.push_entity_unchecked(meta, (A::default(), B::default()));
        assert_eq!(
            archetype.read_components_exact_unchecked::<(A, B)>(idx),
            (A::default(), B::default())
        );
        assert_eq!(
            archetype.read_components_exact_unchecked::<(B, A)>(idx),
            (B::default(), A::default())
        );

        let descriptor = <(A, B, C) as ComponentGroup<'_>>::DESCRIPTOR.archetype();

        let mut archetype = Archetype::new(descriptor);
        let meta = EntityMetadata::default();
        let idx = archetype.push_entity_unchecked(meta, (A::default(), B::default(), C::default()));
        assert_eq!(
            archetype.read_components_exact_unchecked::<(A, B, C)>(idx),
            (A::default(), B::default(), C::default())
        );
        assert_eq!(
            archetype.read_components_exact_unchecked::<(B, C, A)>(idx),
            (B::default(), C::default(), A::default())
        );
    }
}
