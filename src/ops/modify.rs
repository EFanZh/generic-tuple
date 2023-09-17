use crate::ops::fold::{Fold, RFold};
use crate::Tuple;

pub trait Append {
    type Output<T>: SplitLast<Last = T, Rest = Self> + Fold + RFold;

    fn append<T>(self, item: T) -> Self::Output<T>;
}

impl Append for () {
    type Output<T> = (T,);

    fn append<T>(self, item: T) -> Self::Output<T> {
        (item,)
    }
}

pub trait Prepend {
    type Output<T>: SplitFirst<First = T, Rest = Self> + Fold + RFold;

    fn prepend<T>(self, item: T) -> Self::Output<T>;
}

impl Prepend for () {
    type Output<T> = (T,);

    fn prepend<T>(self, item: T) -> Self::Output<T> {
        (item,)
    }
}

pub trait SplitFirst {
    type First;
    type Rest: Tuple + Prepend<Output<Self::First> = Self>;

    fn split_first(self) -> (Self::First, Self::Rest);
}

pub trait SplitLast {
    type Last;
    type Rest: Tuple + Append<Output<Self::Last> = Self>;

    fn split_last(self) -> (Self::Last, Self::Rest);
}

// Macros for implementing `Append` and `Prepend` traits.

macro_rules! impl_extends_helper {
    ([$($all_indices:tt)*] [$($all_types:ident)*]) => {
        impl<$($all_types),*> Append for ($($all_types,)*) {
            type Output<T> = ($($all_types,)* T);

            fn append<T>(self, item: T) -> Self::Output<T> {
                ($(self.$all_indices, )* item)
            }
        }

        impl<$($all_types),*> Prepend for ($($all_types,)*) {
            type Output<T> = (T $(, $all_types)*);

            fn prepend<T>(self, item: T) -> Self::Output<T> {
                (item $(, self.$all_indices)*)
            }
        }
    };
    ([$($all_indices:tt)*] [$($all_types:ident)*] $next_index:tt $next_type:ident $($rest:tt)*) => {
        impl_extends_helper!([$($all_indices)*] [$($all_types)*]);
        impl_extends_helper!([$($all_indices)* $next_index] [$($all_types)* $next_type] $($rest)*);
    };
}

macro_rules! impl_extends {
    ($first_index:tt $first_type:ident $($rest:tt)*) => {
        impl_extends_helper!([$first_index] [$first_type] $($rest)*);
    };
}

// Macros for implementing `SplitFirst` and `SplitLast` traits.

macro_rules! impl_splits_helper {
    (
        $first_index:tt
        [$($non_first_indices:tt)*]
        $last_index:tt
        [$($non_last_indices:tt)*]
        [$($all_types:ident)*]
        $first_type:ident
        [$($non_first_types:ident)*]
        $last_type:ident
        [$($non_last_types:ident)*]
    ) => {
        impl<$($all_types),*> SplitFirst for ($($all_types,)*) {
            type First = $first_type;
            type Rest = ($($non_first_types,)*);

            fn split_first(self) -> (Self::First, Self::Rest) {
                (self.$first_index, ($(self.$non_first_indices,)*))
            }
        }

        impl<$($all_types),*> SplitLast for ($($all_types,)*) {
            type Last = $last_type;
            type Rest = ($($non_last_types,)*);

            fn split_last(self) -> (Self::Last, Self::Rest) {
                (self.$last_index, ($(self.$non_last_indices,)*))
            }
        }
    };
    (
        $first_index:tt
        [$($non_first_indices:tt)*]
        $last_index:tt
        [$($non_last_indices:tt)*]
        [$($all_types:ident)*]
        $first_type:ident
        [$($non_first_types:ident)*]
        $last_type:ident
        [$($non_last_types:ident)*]
        $next_index:tt
        $next_type:ident
        $($rest:tt)*
    ) => {
        impl_splits_helper!(
            $first_index
            [$($non_first_indices)*]
            $last_index
            [$($non_last_indices)*]
            [$($all_types)*]
            $first_type
            [$($non_first_types)*]
            $last_type
            [$($non_last_types)*]
        );

        impl_splits_helper!(
            $first_index
            [$($non_first_indices)* $next_index]
            $next_index
            [$($non_last_indices)* $last_index]
            [$($all_types)* $next_type]
            $first_type
            [$($non_first_types)* $next_type]
            $next_type
            [$($non_last_types)* $last_type]
            $($rest)*
        );
    }
}

macro_rules! impl_splits {
    ($first_index:tt $first_type:ident $($rest:tt)*) => {
        impl_splits_helper!(
            $first_index
            []
            $first_index
            []
            [$first_type]
            $first_type
            []
            $first_type
            []
            $($rest)*
        );
    };
}

// Implement all traits.

macro_rules! impl_all_traits_helper {
    ([$($saved:tt)*] $last_index:tt $last_type:ident) => {
        impl_extends!($($saved)*);
        impl_splits!($($saved)* $last_index $last_type);
    };
    ([$($saved:tt)*] $next_index:tt $next_type:ident $($rest:tt)+) => {
        impl_all_traits_helper!([$($saved)* $next_index $next_type] $($rest)+);
    };
}

macro_rules! impl_all_traits {
    ($(($index:tt, $first_type:ident),)+) => {
        impl_all_traits_helper!([] $($index $first_type)+);
    };
}

impl_all_traits![
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8),
    (9, T9),
    (10, T10),
    (11, T11),
    (12, T12),
    (13, T13),
    (14, T14),
    (15, T15),
    (16, T16),
    (17, T17),
    (18, T18),
    (19, T19),
    (20, T20),
    (21, T21),
    (22, T22),
    (23, T23),
    (24, T24),
    (25, T25),
    (26, T26),
    (27, T27),
    (28, T28),
    (29, T29),
    (30, T30),
    (31, T31),
    (32, T32),
];
