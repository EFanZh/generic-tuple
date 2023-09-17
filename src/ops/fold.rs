use crate::{SplitFirst, SplitLast};

pub trait FoldFnMut {
    type Output<B, T>;

    fn call_mut<B, T>(&mut self, init: B, item: T) -> Self::Output<B, T>;
}

pub trait Fold {
    type Output<B, F>
    where
        F: FoldFnMut;

    fn fold<B, F>(self, init: B, f: &mut F) -> Self::Output<B, F>
    where
        F: FoldFnMut;
}

impl Fold for () {
    type Output<B, F> = B
    where
        F: FoldFnMut;

    fn fold<B, F>(self, init: B, #[allow(unused_variables)] f: &mut F) -> Self::Output<B, F>
    where
        F: FoldFnMut,
    {
        init
    }
}

impl<T> Fold for T
where
    T: SplitLast,
    T::Rest: Fold,
{
    type Output<B, F> = F::Output<<<T as SplitLast>::Rest as Fold>::Output<B, F>, T::Last>
    where
        F: FoldFnMut;

    fn fold<B, F>(self, init: B, f: &mut F) -> Self::Output<B, F>
    where
        F: FoldFnMut,
    {
        let (last, rest) = self.split_last();
        let value = rest.fold(init, f);

        f.call_mut(value, last)
    }
}

pub trait RFold {
    type Output<B, F>
    where
        F: FoldFnMut;

    fn rfold<B, F>(self, init: B, f: &mut F) -> Self::Output<B, F>
    where
        F: FoldFnMut;
}

impl RFold for () {
    type Output<B, F> = B
    where
        F: FoldFnMut;

    fn rfold<B, F>(self, init: B, #[allow(unused_variables)] f: &mut F) -> Self::Output<B, F>
    where
        F: FoldFnMut,
    {
        init
    }
}

impl<T> RFold for T
where
    T: SplitFirst,
    T::Rest: RFold,
{
    type Output<B, F> = F::Output<<<T as SplitFirst>::Rest as RFold>::Output<B, F>, T::First>
    where
        F: FoldFnMut;

    fn rfold<B, F>(self, init: B, f: &mut F) -> Self::Output<B, F>
    where
        F: FoldFnMut,
    {
        let (first, rest) = self.split_first();
        let value = rest.rfold(init, f);

        f.call_mut(value, first)
    }
}
