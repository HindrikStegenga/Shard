use crate::archetype_descriptor::{ArchetypeDescriptor, SizedArchetypeDescriptor};
use crate::archetype_id::ArchetypeId;
use crate::component_descriptor::{ComponentDescriptor, ComponentDescriptorFnPointers};
use crate::{define_component_descriptor, Component};

use private::SealedComponentGroup;

#[macro_export]
macro_rules! expr {
    ($x:expr) => {
        $x
    };
}

#[macro_export]
macro_rules! tuple_index {
    ($tuple:expr, $idx:tt) => {
        expr!($tuple.$idx)
    };
}

pub trait SizedComponentGroup<const N: usize>:
    private::SealedComponentGroup + Sized + 'static
{
    type RefTuple<'c>
    where
        Self: 'c;
    type MutRefTuple<'c>
    where
        Self: 'c;
    type SliceRefTuple<'c>
    where
        Self: 'c;
    type SliceMutRefTuple<'c>
    where
        Self: 'c;

    const GROUP_ID: ArchetypeId =
        ArchetypeDescriptor::compute_archetype_id(<Self as SizedComponentGroup<N>>::DESCRIPTORS);

    const ARCHETYPE: SizedArchetypeDescriptor<N> = SizedArchetypeDescriptor::<N>::new(
        <Self as SizedComponentGroup<N>>::GROUP_ID,
        <Self as SizedComponentGroup<N>>::SORTED_DESCRIPTORS,
    );

    const DESCRIPTORS: &'static [ComponentDescriptor; N];

    const SORTED_DESCRIPTORS: &'static [ComponentDescriptor; N] =
        &ComponentDescriptor::compute_sorted_descriptors(
            <Self as SizedComponentGroup<N>>::DESCRIPTORS,
        );

    unsafe fn get_pointers(&mut self) -> [*mut u8; N];
    unsafe fn get_sorted_pointers(&mut self) -> [*mut u8; N];
}

impl<T: Component + SealedComponentGroup> SizedComponentGroup<1> for T {
    type RefTuple<'c> = &'c T;
    type MutRefTuple<'c> = &'c mut T;

    type SliceRefTuple<'c> = &'c [T];
    type SliceMutRefTuple<'c> = &'c mut [T];

    const DESCRIPTORS: &'static [ComponentDescriptor; 1] = &[define_component_descriptor!(T); 1];

    unsafe fn get_pointers(&mut self) -> [*mut u8; 1] {
        [self as *mut T as *mut u8; 1]
    }

    unsafe fn get_sorted_pointers(&mut self) -> [*mut u8; 1] {
        [self as *mut T as *mut u8; 1]
    }
}

/// Represents a group of components. Used for specifying which component types should be matched in query's.
pub trait ComponentGroup<'c>: private::SealedComponentGroup + Sized + 'static {
    type RefTuple: 'c;
    type MutRefTuple: 'c;

    type SliceRefTuple: 'c;
    type SliceMutRefTuple: 'c;

    /// Amount of component types in the group.
    const LENGTH: u8 = Self::DESCRIPTORS.len() as u8;

    /// Unique ID of the group.
    const GROUP_ID: ArchetypeId =
        ArchetypeDescriptor::compute_archetype_id(Self::SORTED_DESCRIPTORS);
    /// Archetype descriptor representing the group.
    const ARCHETYPE_DESCRIPTOR: ArchetypeDescriptor;

    /// Descriptors of the components according to the group's type's ordering.
    const DESCRIPTORS: &'static [ComponentDescriptor];
    /// Descriptors of the components sorted by their id's. The ECS stores groups internally in this order!
    const SORTED_DESCRIPTORS: &'static [ComponentDescriptor];
}

impl<'s, T: Component + SealedComponentGroup> ComponentGroup<'s> for T {
    type RefTuple = &'s T;
    type MutRefTuple = &'s mut T;

    type SliceRefTuple = &'s [T];
    type SliceMutRefTuple = &'s mut [T];

    const ARCHETYPE_DESCRIPTOR: ArchetypeDescriptor = ArchetypeDescriptor::new(
        <Self as ComponentGroup<'s>>::GROUP_ID,
        Self::LENGTH,
        unsafe {
            ComponentDescriptor::write_into_fixed_size_array(
                <Self as ComponentGroup<'s>>::SORTED_DESCRIPTORS,
            )
        },
    );

    const DESCRIPTORS: &'static [ComponentDescriptor] = &[define_component_descriptor!(T)];
    const SORTED_DESCRIPTORS: &'static [ComponentDescriptor] = &[define_component_descriptor!(T)];
}

macro_rules! impl_component_tuple {
    ($len:expr, $(($elem:ident, $elem_idx:tt)), *) => {
        impl<'s, $($elem),*> ComponentGroup<'s> for ($($elem), *)
        where $( $elem : Component + SealedComponentGroup ),*, Self : SealedComponentGroup
        {
            type RefTuple = ($(&'s $elem),*);
            type MutRefTuple = ($(&'s mut $elem),*);

            type SliceRefTuple = ($(&'s [$elem]),*);
            type SliceMutRefTuple = ($(&'s mut [$elem]),*);

            const DESCRIPTORS: &'static [ComponentDescriptor] = { let s = &[
                $( define_component_descriptor!($elem)), *
            ]; ComponentDescriptor::validate_descriptors(s, false, &[]) };

            const SORTED_DESCRIPTORS: &'static [ComponentDescriptor] = {
                let s = [ $( define_component_descriptor!($elem)), * ];
                if ComponentDescriptor::validate_descriptors(&s, false, &[]).is_empty() { &[] } else {
                    &ComponentDescriptor::compute_sorted_descriptors(&[ $( define_component_descriptor!($elem)), * ])
                }
            };

            const ARCHETYPE_DESCRIPTOR : ArchetypeDescriptor = ArchetypeDescriptor::new(
                <Self as ComponentGroup<'s>>::GROUP_ID,
                Self::LENGTH,
                unsafe { ComponentDescriptor::write_into_fixed_size_array(<Self as ComponentGroup<'s>>::SORTED_DESCRIPTORS) }
            );
        }

        impl<$($elem),*> SizedComponentGroup<$len> for ($($elem), *)
        where $( $elem : Component + SealedComponentGroup ),*, Self : SealedComponentGroup
        {
            type RefTuple<'s> = ($(&'s $elem),*);
            type MutRefTuple<'s> = ($(&'s mut $elem),*);

            type SliceRefTuple<'s> = ($(&'s [$elem]),*);
            type SliceMutRefTuple<'s> = ($(&'s mut [$elem]),*);

            const DESCRIPTORS: &'static [ComponentDescriptor; $len] = { let s = &[
                $( define_component_descriptor!($elem)), *
            ]; ComponentDescriptor::validate_descriptors_fixed(s, false) };

            const SORTED_DESCRIPTORS: &'static [ComponentDescriptor; $len] = {
                let s = [ $( define_component_descriptor!($elem)), * ];
                //if ComponentDescriptor::validate_descriptors_fixed(&s, false).is_empty() { &[] } else {
                    &ComponentDescriptor::compute_sorted_descriptors(&[ $( define_component_descriptor!($elem)), * ])
                //}
            };

            unsafe fn get_pointers(&mut self) -> [*mut u8; $len] {
                let mut v = [core::ptr::null_mut::<u8>(); $len];
                $(
                    *v.get_unchecked_mut($elem_idx) = &mut tuple_index!(self, $elem_idx) as *mut $elem as *mut u8;
                )*
                return v;
            }

            unsafe fn get_sorted_pointers(&mut self) -> [*mut u8; $len] {
                todo!()
                // let mut v = [core::ptr::null_mut::<u8>(); $len];
                // $(
                //     *v.get_unchecked_mut(*Self::UNSORTED_TO_SORTED_MAPPING_INDICES.get_unchecked($elem_idx) as usize) = &mut tuple_index!(self, $elem_idx) as *mut $elem as *mut u8;
                // )*
                // return v;
            }
        }
    }
}

impl_component_tuple!(
    16,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8),
    (T10, 9),
    (T11, 10),
    (T12, 11),
    (T13, 12),
    (T14, 13),
    (T15, 14),
    (T16, 15)
);

impl_component_tuple!(
    15,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8),
    (T10, 9),
    (T11, 10),
    (T12, 11),
    (T13, 12),
    (T14, 13),
    (T15, 14)
);

impl_component_tuple!(
    14,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8),
    (T10, 9),
    (T11, 10),
    (T12, 11),
    (T13, 12),
    (T14, 13)
);

impl_component_tuple!(
    13,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8),
    (T10, 9),
    (T11, 10),
    (T12, 11),
    (T13, 12)
);

impl_component_tuple!(
    12,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8),
    (T10, 9),
    (T11, 10),
    (T12, 11)
);

impl_component_tuple!(
    11,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8),
    (T10, 9),
    (T11, 10)
);

impl_component_tuple!(
    10,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8),
    (T10, 9)
);

impl_component_tuple!(
    9,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7),
    (T9, 8)
);

impl_component_tuple!(
    8,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6),
    (T8, 7)
);
impl_component_tuple!(
    7,
    (T1, 0),
    (T2, 1),
    (T3, 2),
    (T4, 3),
    (T5, 4),
    (T6, 5),
    (T7, 6)
);

impl_component_tuple!(6, (T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5));
impl_component_tuple!(5, (T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4));
impl_component_tuple!(4, (T1, 0), (T2, 1), (T3, 2), (T4, 3));
impl_component_tuple!(3, (T1, 0), (T2, 1), (T3, 2));
impl_component_tuple!(2, (T1, 0), (T2, 1));

mod private {
    use crate::Component;

    pub trait SealedComponentGroup {}

    impl<'s, T> SealedComponentGroup for T where T: Component {}

    macro_rules! impl_sealed_component_tuples {
        ($($ty:ident)*) => {}; //base case
        ($head:ident, $($tail:ident),*) => {
            impl<$($tail),*, $head> SealedComponentGroup for impl_sealed_component_tuples!([$($tail)*] $head)
            where $head : Component, $( $tail : Component ),* {}

            impl_sealed_component_tuples!($($tail),*);
        };
        ([] $($ty:ident)*) => {
            ($($ty), *)
        };
        ([$first:ident $($tail:ident)*] $($ty:ident)*) => {
            impl_sealed_component_tuples!([$($tail)*] $first $($ty)*)
        };
    }

    impl_sealed_component_tuples!(
        T16, T15, T14, T13, T12, T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1
    );
}
