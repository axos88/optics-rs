use crate::{
    FallibleIso, HasGetter, HasReverseGet, HasSetter, Iso, IsoImpl, Lens, LensImpl, Prism,
    PrismImpl, composed_fallible_iso, composed_prism, infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

pub struct FallibleIsoImpl<S, A, FI: FallibleIso<S, A>>(pub FI, PhantomData<(S, A)>);

impl<S, A, FI: FallibleIso<S, A>> FallibleIsoImpl<S, A, FI> {
    pub fn new(l: FI) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        FallibleIsoImpl(l, PhantomData)
    }
}

impl<S, A, FI: FallibleIso<S, A>> HasGetter<S, A> for FallibleIsoImpl<S, A, FI> {
    type GetterError = FI::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, A, FI: FallibleIso<S, A>> HasSetter<S, A> for FallibleIsoImpl<S, A, FI> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, FI: FallibleIso<S, A>> HasReverseGet<S, A> for FallibleIsoImpl<S, A, FI> {
    type ReverseError = FI::ReverseError;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        self.0.try_reverse_get(value)
    }
}

impl<S, I, FI1: FallibleIso<S, I>> FallibleIsoImpl<S, I, FI1> {
    pub fn compose_with_prism<E, A, P2: Prism<I, A>>(
        self,
        other: P2,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>>
    where
        E: From<FI1::GetterError> + From<P2::GetterError>,
    {
        composed_prism(self, other, Into::into, Into::into)
    }

    pub fn compose_with_prism_with_mappers<E, A, P2: Prism<I, A>>(
        self,
        other: P2,
        error_mapper_1: fn(FI1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>> {
        composed_prism(self, other, error_mapper_1, error_mapper_2)
    }

    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = FI1::GetterError>> {
        composed_prism(self, other, identity, infallible)
    }

    pub fn compose_with_fallible_iso<GE, RE, A, FI2: FallibleIso<I, A>>(
        self,
        other: FI2,
    ) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A, GetterError = GE, ReverseError = RE>>
    where
        GE: From<FI1::GetterError> + From<FI2::GetterError>,
        RE: From<FI1::ReverseError> + From<FI2::ReverseError>,
    {
        composed_fallible_iso(self, other, Into::into, Into::into, Into::into, Into::into)
    }

    pub fn compose_with_fallible_iso_with_mappers<GE, RE, A, FI2: FallibleIso<I, A>>(
        self,
        other: FI2,
        getter_error_mapper_1: fn(FI1::GetterError) -> GE,
        getter_error_mapper_2: fn(FI2::GetterError) -> GE,
        reverse_error_mapper_1: fn(FI1::ReverseError) -> RE,
        reverse_error_mapper_2: fn(FI2::ReverseError) -> RE,
    ) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A, GetterError = GE, ReverseError = RE>> {
        composed_fallible_iso(
            self,
            other,
            getter_error_mapper_1,
            getter_error_mapper_2,
            reverse_error_mapper_1,
            reverse_error_mapper_2,
        )
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A>> {
        composed_fallible_iso(self.0, other.0, identity, infallible, identity, infallible)
    }
}
