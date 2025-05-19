use crate::{
    FallibleIso, FallibleIsoImpl, HasGetter, HasPartialGetter, HasPartialReversible, HasReversible,
    HasSetter, Iso, Lens, LensImpl, Prism, PrismImpl, composed_fallible_iso, composed_iso,
    composed_lens, composed_prism, infallible,
};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

pub struct IsoImpl<S, A, ISO: Iso<S, A>>(pub ISO, PhantomData<(S, A)>);

impl<S, A, ISO: Iso<S, A>> IsoImpl<S, A, ISO> {
    fn new(i: ISO) -> Self {
        IsoImpl(i, PhantomData)
    }
}

impl<S, A, ISO: Iso<S, A>> From<ISO> for IsoImpl<S, A, ISO> {
    fn from(value: ISO) -> Self {
        Self::new(value)
    }
}

impl<S, A, ISO: Iso<S, A>> HasPartialGetter<S, A> for IsoImpl<S, A, ISO> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, A, ISO: Iso<S, A>> HasGetter<S, A> for IsoImpl<S, A, ISO> {
    fn get(&self, source: &S) -> A {
        self.0.get(source)
    }
}

impl<S, A, ISO: Iso<S, A>> HasSetter<S, A> for IsoImpl<S, A, ISO> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, ISO: Iso<S, A>> HasPartialReversible<S, A> for IsoImpl<S, A, ISO> {
    type ReverseError = Infallible;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        Ok(self.0.reverse_get(value))
    }
}

impl<S, A, ISO: Iso<S, A>> HasReversible<S, A> for IsoImpl<S, A, ISO> {
    fn reverse_get(&self, value: &A) -> S {
        self.0.reverse_get(value)
    }
}

impl<S, I, ISO1: Iso<S, I>> IsoImpl<S, I, ISO1> {
    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> LensImpl<S, A, impl Lens<S, A>> {
        composed_lens(self, other)
    }

    pub fn compose_with_prism<A, P2: Prism<I, A>>(
        self,
        other: P2,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = P2::GetterError>> {
        composed_prism(self, other, infallible, identity)
    }

    pub fn compose_with_fallible_iso<A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> FallibleIsoImpl<
        S,
        A,
        impl FallibleIso<S, A, GetterError = F2::GetterError, ReverseError = F2::ReverseError>,
    > {
        composed_fallible_iso(self, other, infallible, identity, infallible, identity)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> IsoImpl<S, A, impl Iso<S, A>> {
        composed_iso(self.0, other.0)
    }
}
