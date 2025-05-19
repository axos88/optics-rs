use crate::partial_getter::composed::new as composed_partial_getter;
use crate::{
    FallibleIso, FallibleIsoImpl, Getter, GetterImpl, HasGetter, Iso, IsoImpl, Lens, LensImpl,
    PartialGetter, Prism, PrismImpl, infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

pub struct PartialGetterImpl<S, A, PG: PartialGetter<S, A>>(pub PG, PhantomData<(S, A)>);

impl<S, A, PG: PartialGetter<S, A>> PartialGetterImpl<S, A, PG> {
    fn new(prism: PG) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        PartialGetterImpl(prism, PhantomData)
    }
}

impl<S, A, PG: PartialGetter<S, A>> From<PG> for PartialGetterImpl<S, A, PG> {
    fn from(value: PG) -> Self {
        Self::new(value)
    }
}

impl<S, A, PG: PartialGetter<S, A>> HasGetter<S, A> for PartialGetterImpl<S, A, PG> {
    type GetterError = PG::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, I, PG1: PartialGetter<S, I>> PartialGetterImpl<S, I, PG1> {
    pub fn compose_with_partial_getter<E, A, PG2: PartialGetter<I, A>>(
        self,
        other: PG2,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        PG1::GetterError: Into<E>,
        PG2::GetterError: Into<E>,
    {
        composed_partial_getter(self, other, Into::into, Into::into)
    }

    pub fn compose_with_partial_getter_with_mappers<E, A, PG2: PartialGetter<I, A>>(
        self,
        other: PG2,
        error_mapper1: fn(PG1::GetterError) -> E,
        error_mapper_2: fn(PG2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other, error_mapper1, error_mapper_2)
    }

    pub fn compose_with_getter<A, G2: Getter<I, A>>(
        self,
        other: GetterImpl<I, A, G2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other, identity, infallible)
    }

    pub fn compose_with_prism<E, A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        PG1::GetterError: Into<E>,
        P2::GetterError: Into<E>,
    {
        composed_partial_getter(self, other.0, Into::into, Into::into)
    }

    pub fn compose_with_prism_with_mappers<E, A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
        error_mapper_1: fn(PG1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(
            self,
            PartialGetterImpl::new(other.0),
            error_mapper_1,
            error_mapper_2,
        )
    }

    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other.0, identity, infallible)
    }

    pub fn compose_with_fallible_iso<E, A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        E: From<FI2::GetterError> + From<PG1::GetterError>,
    {
        composed_partial_getter(self, other.0, Into::<E>::into, Into::<E>::into)
    }

    pub fn compose_with_fallible_iso_with_mappers<E, A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
        getter_error_mapper_1: fn(PG1::GetterError) -> E,
        getter_error_mapper_2: fn(FI2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self, other.0, getter_error_mapper_1, getter_error_mapper_2)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = PG1::GetterError>> {
        composed_partial_getter(self, other.0, identity, infallible)
    }
}
