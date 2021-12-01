use crate::descriptors::component_descriptor::{ComponentDescriptor, ComponentDescriptorFnPointers};
use crate::{define_component_descriptor, Component, MAX_COMPONENTS_PER_ENTITY};

use crate::descriptors::component_group_descriptor::ComponentGroupDescriptor;
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

/// Represents a group of components. Used for specifying which component types should be matched in query's.
pub trait ComponentGroup<'c>: private::SealedComponentGroup + Sized + 'static {
    type RefTuple: 'c;
    type MutRefTuple: 'c;
    type SliceRefTuple: 'c;
    type SliceMutRefTuple: 'c;

    /// The descriptor which exactly specifies all components of the component group.
    const DESCRIPTOR: ComponentGroupDescriptor;

    /// Returns the sorted pointers given a reference to self.
    unsafe fn as_sorted_pointers(&mut self, ptrs: &mut [*mut u8; MAX_COMPONENTS_PER_ENTITY]);

    /// Returns an instance of self, read from the sorted pointers.
    unsafe fn read_from_sorted_pointers(pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY]) -> Self;

    /// Returns a reference tuple of component types given an array of sorted pointers.
    unsafe fn pointers_as_ref_tuple(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
    ) -> Self::RefTuple;

    /// Returns a mutable reference tuple of component types given an array of sorted pointers.
    unsafe fn pointers_as_mut_ref_tuple(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
    ) -> Self::MutRefTuple;

    /// Returns a tuple of slices extracted from the given pointers.
    /// # Safety:
    /// - The pointers must be sorted.
    unsafe fn slice_unchecked(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
        len: usize,
    ) -> Self::SliceRefTuple;

    /// Returns a tuple of mutable slices extracted from the given pointers.
    /// # Safety:
    /// - The pointers must be sorted.
    unsafe fn slice_unchecked_mut(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
        len: usize,
    ) -> Self::SliceMutRefTuple;

    /// Returns an tuple of empty slices.
    fn empty_slice() -> Self::SliceRefTuple;

    /// Returns an tuple of empty mutable slices.
    fn empty_slice_mut() -> Self::SliceMutRefTuple;
}

impl<'c, T: Component + SealedComponentGroup> ComponentGroup<'c> for T {
    type RefTuple = &'c T;
    type MutRefTuple = &'c mut T;

    type SliceRefTuple = &'c [T];
    type SliceMutRefTuple = &'c mut [T];

    const DESCRIPTOR: ComponentGroupDescriptor =
        ComponentGroupDescriptor::new(&[define_component_descriptor!(T)]);

    unsafe fn as_sorted_pointers(&mut self, ptrs: &mut [*mut u8; MAX_COMPONENTS_PER_ENTITY]) {
        ptrs[0] = self as *mut T as *mut u8;
    }

    unsafe fn read_from_sorted_pointers(pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY]) -> Self {
        core::ptr::read(pointers[0] as *mut T)
    }

    unsafe fn pointers_as_ref_tuple(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
    ) -> Self::RefTuple {
        &*(sorted_pointers[0] as *mut T)
    }

    unsafe fn pointers_as_mut_ref_tuple(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
    ) -> Self::MutRefTuple {
        &mut *(sorted_pointers[0] as *mut T)
    }

    unsafe fn slice_unchecked(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
        len: usize,
    ) -> Self::SliceRefTuple {
        core::slice::from_raw_parts(sorted_pointers[0] as *const T, len)
    }

    unsafe fn slice_unchecked_mut(
        sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
        len: usize,
    ) -> Self::SliceMutRefTuple {
        core::slice::from_raw_parts_mut(sorted_pointers[0] as *mut T, len)
    }

    fn empty_slice() -> Self::SliceRefTuple {
        &[]
    }

    fn empty_slice_mut() -> Self::SliceMutRefTuple {
        &mut []
    }
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

            fn empty_slice() -> Self::SliceRefTuple {
                ($(&[] as &[$elem]), *)
            }

            fn empty_slice_mut() -> Self::SliceMutRefTuple {
                ($(&mut [] as &mut [$elem]), *)
            }

            const DESCRIPTOR: ComponentGroupDescriptor =
                ComponentGroupDescriptor::new(&[$(define_component_descriptor!($elem)), *]);

            unsafe fn as_sorted_pointers(&mut self, ptrs: &mut [*mut u8; MAX_COMPONENTS_PER_ENTITY]) {
                $(
                    ptrs[Self::DESCRIPTOR.unsorted_to_sorted($elem_idx) as usize] = &mut tuple_index!(self, $elem_idx) as *mut $elem as *mut u8;
                )*
            }

            unsafe fn read_from_sorted_pointers(pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY]) -> Self {
                ($(
                    core::ptr::read(pointers[Self::DESCRIPTOR.unsorted_to_sorted($elem_idx) as usize] as *mut $elem)
                ),*)
            }

            unsafe fn pointers_as_ref_tuple(
                sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
            ) -> Self::RefTuple {
                ($(
                    &*((sorted_pointers[Self::DESCRIPTOR.unsorted_to_sorted($elem_idx) as usize]) as *mut $elem),
                )*)
            }

            unsafe fn pointers_as_mut_ref_tuple(
                sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
            ) -> Self::MutRefTuple {
                ($(
                    &mut *((sorted_pointers[Self::DESCRIPTOR.unsorted_to_sorted($elem_idx) as usize]) as *mut $elem),
                )*)
            }

            unsafe fn slice_unchecked(
                sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
                len: usize,
            ) -> Self::SliceRefTuple {
                ($(
                    core::slice::from_raw_parts(sorted_pointers[Self::DESCRIPTOR.unsorted_to_sorted($elem_idx) as usize] as *const $elem, len),
                )*)
            }

            unsafe fn slice_unchecked_mut(
                sorted_pointers: &[*mut u8; MAX_COMPONENTS_PER_ENTITY],
                len: usize,
            ) -> Self::SliceMutRefTuple {
                                ($(
                    core::slice::from_raw_parts_mut(sorted_pointers[Self::DESCRIPTOR.unsorted_to_sorted($elem_idx) as usize] as *mut $elem, len),
                )*)
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use super::*;
    use crate::test_components::*;

    #[test]
    fn test_component_group_as_sorted_pointers() {
        unsafe {
            let mut group = (A::default(), B::default(), C::default());

            let mut ptrs = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
            ComponentGroup::as_sorted_pointers(&mut group, &mut ptrs);
            assert_eq!(ptrs[0], &mut group.0 as *mut A as *mut u8);
            assert_eq!(ptrs[1], &mut group.1 as *mut B as *mut u8);
            assert_eq!(ptrs[2], &mut group.2 as *mut C as *mut u8);

            let mut group = (B::default(), C::default(), A::default());
            let mut ptrs = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
            ComponentGroup::as_sorted_pointers(&mut group, &mut ptrs);
            assert_eq!(ptrs[0], &mut group.2 as *mut A as *mut u8);
            assert_eq!(ptrs[1], &mut group.0 as *mut B as *mut u8);
            assert_eq!(ptrs[2], &mut group.1 as *mut C as *mut u8);
        }
    }

    #[test]
    fn test_component_group_sorted_pointers_as_ref_tuples() {
        unsafe {
            let mut group = (A::default(), B::default(), C::default());

            let mut ptrs = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
            ComponentGroup::as_sorted_pointers(&mut group, &mut ptrs);

            let result = <(A, B, C) as ComponentGroup<'_>>::pointers_as_ref_tuple(&ptrs);
            assert_eq!(result.0 as *const A as *mut u8, ptrs[0]);
            assert_eq!(result.1 as *const B as *mut u8, ptrs[1]);
            assert_eq!(result.2 as *const C as *mut u8, ptrs[2]);

            let result = <(A, B, C) as ComponentGroup<'_>>::pointers_as_mut_ref_tuple(&ptrs);
            assert_eq!(result.0 as *mut A as *mut u8, ptrs[0]);
            assert_eq!(result.1 as *mut B as *mut u8, ptrs[1]);
            assert_eq!(result.2 as *mut C as *mut u8, ptrs[2]);

            let result = <(B, C, A) as ComponentGroup<'_>>::pointers_as_ref_tuple(&ptrs);
            assert_eq!(result.0 as *const B as *mut u8, ptrs[1]);
            assert_eq!(result.1 as *const C as *mut u8, ptrs[2]);
            assert_eq!(result.2 as *const A as *mut u8, ptrs[0]);
            let result = <(C, A, B) as ComponentGroup<'_>>::pointers_as_mut_ref_tuple(&ptrs);
            assert_eq!(result.0 as *mut C as *mut u8, ptrs[2]);
            assert_eq!(result.1 as *mut A as *mut u8, ptrs[0]);
            assert_eq!(result.2 as *mut B as *mut u8, ptrs[1]);
        }
    }

    #[test]
    fn test_component_group_sorted_pointers_to_tuples() {
        unsafe {
            let mut group = (A::default(), B::default(), C::default());

            let mut ptrs = [core::ptr::null_mut(); MAX_COMPONENTS_PER_ENTITY];
            ComponentGroup::as_sorted_pointers(&mut group, &mut ptrs);

            let result = <(A, B, C) as ComponentGroup<'_>>::read_from_sorted_pointers(&ptrs);
            assert_eq!(result.0, group.0);
            assert_eq!(result.1, group.1);
            assert_eq!(result.2, group.2);

            let result = <(C, A, B) as ComponentGroup<'_>>::read_from_sorted_pointers(&ptrs);
            assert_eq!(result.1, group.0);
            assert_eq!(result.2, group.1);
            assert_eq!(result.0, group.2);
        }
    }

    #[test]
    fn test_component_group_slices() {
        unsafe {
            let mut slice_a = [
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
                A::default(),
            ];
            let mut slice_b = [
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
                B::default(),
            ];
            let mut slice_c = [
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
                C::default(),
            ];

            let mut pointers = [core::ptr::null_mut::<u8>(); MAX_COMPONENTS_PER_ENTITY];
            pointers[0] = slice_a.as_mut_ptr() as *mut u8;
            pointers[1] = slice_b.as_mut_ptr() as *mut u8;
            pointers[2] = slice_c.as_mut_ptr() as *mut u8;

            let slices: (&[A], &[B], &[C]) =
                <(A, B, C) as ComponentGroup<'_>>::slice_unchecked(&mut pointers, slice_a.len());
            assert_eq!(slices.0.as_ptr() as *mut u8, pointers[0]);
            assert_eq!(slices.1.as_ptr() as *mut u8, pointers[1]);
            assert_eq!(slices.2.as_ptr() as *mut u8, pointers[2]);

            let slices: (&[B], &[C], &[A]) =
                <(B, C, A) as ComponentGroup<'_>>::slice_unchecked(&mut pointers, slice_a.len());
            assert_eq!(slices.0.as_ptr() as *mut u8, pointers[1]);
            assert_eq!(slices.1.as_ptr() as *mut u8, pointers[2]);
            assert_eq!(slices.2.as_ptr() as *mut u8, pointers[0]);
        }
    }
}

// impl_component_tuple!(
//     16,
//     (T1, 0),
//     (T2, 1),
//     (T3, 2),
//     (T4, 3),
//     (T5, 4),
//     (T6, 5),
//     (T7, 6),
//     (T8, 7),
//     (T9, 8),
//     (T10, 9),
//     (T11, 10),
//     (T12, 11),
//     (T13, 12),
//     (T14, 13),
//     (T15, 14),
//     (T16, 15)
// );
//
// impl_component_tuple!(
//     15,
//     (T1, 0),
//     (T2, 1),
//     (T3, 2),
//     (T4, 3),
//     (T5, 4),
//     (T6, 5),
//     (T7, 6),
//     (T8, 7),
//     (T9, 8),
//     (T10, 9),
//     (T11, 10),
//     (T12, 11),
//     (T13, 12),
//     (T14, 13),
//     (T15, 14)
// );

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Component;
    struct Position;
    struct Rotation;
    struct Velocity;
    impl Component for Position {
        const NAME: &'static str = "Position";
    }
    impl Component for Rotation {
        const NAME: &'static str = "Rotation";
    }
    impl Component for Velocity {
        const NAME: &'static str = "Velocity";
    }

    #[test]
    fn test_component_group_len() {
        fn test_group_len<'c, G: ComponentGroup<'c>>(expected_len: usize) {
            assert_eq!(G::DESCRIPTOR.archetype().len() as usize, expected_len);
        }

        test_group_len::<Position>(1);
        test_group_len::<(Position, Rotation)>(2);
        test_group_len::<(Position, Rotation, Velocity)>(3);
    }

    #[test]
    fn test_component_group_descriptor() {
        #[cfg(test)]
        extern crate std;

        assert!(<Position as ComponentGroup>::DESCRIPTOR.is_valid());
        assert!(<(Position, Position) as ComponentGroup>::DESCRIPTOR.is_valid() == false);
        assert!(<(Position, Rotation) as ComponentGroup>::DESCRIPTOR.is_valid());
    }
}
