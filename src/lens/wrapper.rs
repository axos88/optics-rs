use crate::{
    FallibleIso, FallibleIsoImpl, HasGetter, HasPartialGetter, HasSetter, Iso, IsoImpl, Lens,
    Prism, PrismImpl, composed_lens, composed_prism, infallible,
};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

pub struct LensImpl<S, A, L: Lens<S, A>>(pub L, PhantomData<(S, A)>);

impl<S, A, L: Lens<S, A>> LensImpl<S, A, L> {
    fn new(l: L) -> Self {
        LensImpl(l, PhantomData)
    }
}

impl<S, A, L: Lens<S, A>> From<L> for LensImpl<S, A, L> {
    fn from(value: L) -> Self {
        Self::new(value)
    }
}

impl<S, A, L: Lens<S, A>> HasPartialGetter<S, A> for LensImpl<S, A, L> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, A, L: Lens<S, A>> HasSetter<S, A> for LensImpl<S, A, L> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, I, L: Lens<S, I>> LensImpl<S, I, L> {
    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> LensImpl<S, A, impl Lens<S, A>> {
        composed_lens(self.0, other.0)
    }

    pub fn compose_with_prism<A, P: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = P::GetterError>> {
        composed_prism(self.0, other.0, infallible, identity)
    }

    pub fn compose_with_fallible_iso<A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = F2::GetterError>> {
        composed_prism(self, other, infallible, identity)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> LensImpl<S, A, impl Lens<S, A>> {
        composed_lens(self.0, other.0)
    }
}
