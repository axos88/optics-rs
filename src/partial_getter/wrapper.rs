use crate::partial_getter::composed::new as composed_partial_getter;
use crate::{
    FallibleIso, FallibleIsoImpl, HasPartialGetter, Iso, IsoImpl, Lens, LensImpl, PartialGetter,
    Prism, PrismImpl, infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

pub struct PartialGetterImpl<S, A, P: PartialGetter<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, P: PartialGetter<S, A>> PartialGetterImpl<S, A, P> {
    fn new(prism: P) -> Self {
        PartialGetterImpl(prism, PhantomData)
    }
}

impl<S, A, PG: PartialGetter<S, A>> From<PG> for PartialGetterImpl<S, A, PG> {
    fn from(value: PG) -> Self {
        Self::new(value)
    }
}

impl<S, A, P: PartialGetter<S, A>> HasPartialGetter<S, A> for PartialGetterImpl<S, A, P> {
    type GetterError = P::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, I, P1: PartialGetter<S, I>> PartialGetterImpl<S, I, P1> {
    pub fn compose_with_partial_getter<E, A, P2: PartialGetter<I, A>>(
        self,
        other: P2,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        P1::GetterError: Into<E>,
        P2::GetterError: Into<E>,
    {
        composed_partial_getter(self, other, Into::into, Into::into)
    }

    pub fn compose_with_partial_getter_with_mappers<E, A, P2: PartialGetter<I, A>>(
        self,
        other: P2,
        error_mapper1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other, error_mapper1, error_mapper_2)
    }

    pub fn compose_with_getter<A, O2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, O2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other, identity, infallible)
    }

    pub fn compose_with_prism<E, A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        P1::GetterError: Into<E>,
        P2::GetterError: Into<E>,
    {
        composed_partial_getter(self, other.0, Into::into, Into::into)
    }

    pub fn compose_with_prism_with_mappers<E, A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
        error_mapper_1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(
            self,
            PartialGetterImpl::new(other.0),
            error_mapper_1,
            error_mapper_2,
        )
    }

    pub fn compose_with_lens<A, O2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, O2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other.0, identity, infallible)
    }

    pub fn compose_with_fallible_iso<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        E: From<F2::GetterError> + From<P1::GetterError>,
    {
        composed_partial_getter(self, other.0, Into::<E>::into, Into::<E>::into)
    }

    pub fn compose_with_fallible_iso_with_mappers<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
        getter_error_mapper_1: fn(P1::GetterError) -> E,
        getter_error_mapper_2: fn(F2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other.0, getter_error_mapper_1, getter_error_mapper_2)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = P1::GetterError>> {
        composed_partial_getter(self, other.0, identity, infallible)
    }
}
