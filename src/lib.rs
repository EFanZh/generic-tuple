use self::ops::{Append, Fold, FoldFnMut, Prepend, RFold, SplitFirst, SplitLast};

pub mod ops;

pub trait Tuple: Append + Prepend + Fold + RFold {
    fn append<T>(self, item: T) -> <Self as Append>::Output<T>
    where
        Self: Sized,
    {
        Append::append(self, item)
    }

    fn prepend<T>(self, item: T) -> <Self as Prepend>::Output<T>
    where
        Self: Sized,
    {
        Prepend::prepend(self, item)
    }

    fn split_first(self) -> (Self::First, Self::Rest)
    where
        Self: SplitFirst + Sized,
    {
        SplitFirst::split_first(self)
    }

    fn split_last(self) -> (Self::Last, Self::Rest)
    where
        Self: SplitLast + Sized,
    {
        SplitLast::split_last(self)
    }

    fn fold<B, F>(self, init: B, mut f: F) -> <Self as Fold>::Output<B, F>
    where
        Self: Sized,
        F: FoldFnMut,
    {
        Fold::fold(self, init, &mut f)
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> <Self as RFold>::Output<B, F>
    where
        Self: Sized,
        F: FoldFnMut,
    {
        RFold::rfold(self, init, &mut f)
    }
}

impl<T> Tuple for T where T: Append + Prepend + Fold + RFold {}
