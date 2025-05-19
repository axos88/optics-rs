use crate::getter::composed::new as composed_getter;
use crate::{
    FallibleIso, FallibleIsoImpl, Getter, HasGetter, HasPartialGetter, Iso, IsoImpl, Lens,
    LensImpl, PartialGetter, PartialGetterImpl, Prism, PrismImpl, composed_partial_getter,
    infallible,
};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

pub struct GetterImpl<S, A, P: Getter<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, G: Getter<S, A>> From<G> for GetterImpl<S, A, G> {
    fn from(value: G) -> Self {
        Self::new(value)
    }
}

impl<S, A, G: Getter<S, A>> GetterImpl<S, A, G> {
    fn new(prism: G) -> Self {
        GetterImpl(prism, PhantomData)
    }
}

impl<S, A, P: Getter<S, A>> HasPartialGetter<S, A> for GetterImpl<S, A, P> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, I, G1: Getter<S, I>> GetterImpl<S, I, G1> {
    pub fn compose_with_getter<A, G2: Getter<I, A>>(
        self,
        other: GetterImpl<I, A, G2>,
    ) -> GetterImpl<S, A, impl Getter<S, A>> {
        composed_getter(self.0, other.0)
    }

    pub fn compose_with_prism<A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = P2::GetterError>> {
        composed_partial_getter(self, other, infallible, identity)
    }

    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> GetterImpl<S, A, impl Getter<S, A>> {
        composed_getter(self, other.0)
    }

    pub fn compose_with_fallible_iso<A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = F2::GetterError>> {
        composed_partial_getter(self, other.0, infallible, identity)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> GetterImpl<S, A, impl Getter<S, A>> {
        composed_getter(self, other.0)
    }
}
