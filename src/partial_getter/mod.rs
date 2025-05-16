use crate::HasPartialGetter;
use crate::fallible_iso::{FallibleIso, FallibleIsoImpl};
use crate::lens::{Lens, LensImpl};
use crate::partial_getter::composed::ComposedPartialGetter;
use crate::{Iso, IsoImpl, infallible};
use core::convert::identity;
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
pub use composed::new as composed_partial_getter;
pub use mapped::new as mapped_partial_getter;

/// A `PartialGetter` is an optic that focuses on a potential value inside a sum type.
///
/// It provides:
/// - `preview` to optionally extract a focus value from a larger type
/// - `set` to construct the larger type from a focus value
///
/// This is useful for working with `enum` variants, `Option` values, or
/// other sum types where a focus value might be present.
///
/// Be very careful if you intend to implement this trait yourself, it should not be needed.
///
/// # Note
///
/// `PartialGetter` is currently the most general form of a *concrete optic* in this library.
/// In the future, it may be generalized further to allow any error type in place
/// of the fixed `NoFocus` error. When that happens, `PartialGetter<S, A>` will become a
/// special case of a fully general `Optic<S, A>` abstraction, making this trait
/// redundant and subject to removal in favor of the unified `Optic<S, A>` design.
///
/// This would allow error type specialization while preserving the same core behavior.
///
/// # See Also
/// - [`Optic`] — the base trait for all optic types, potentially unifying `Lens`, `PartialGetter`, and `Iso`
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the forward mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible one-to-one transformation between two types
///
/// - [`NoFocus`] — the current error type returned by `PartialGetter::preview` on failure
pub trait PartialGetter<S, A>: HasPartialGetter<S, A> {}

pub struct PartialGetterImpl<S, A, P: PartialGetter<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, P: PartialGetter<S, A>> PartialGetterImpl<S, A, P> {
    pub fn new(prism: P) -> Self {
        PartialGetterImpl(prism, PhantomData)
    }
}

impl<S, A, P: PartialGetter<S, A>> HasPartialGetter<S, A> for PartialGetterImpl<S, A, P> {
    type GetterError = P::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, A, P: PartialGetter<S, A>> PartialGetter<S, A> for PartialGetterImpl<S, A, P> {}

impl<S, I, P1: PartialGetter<S, I>> PartialGetterImpl<S, I, P1> {
    pub fn compose_with_partial_getter<
        E: From<P1::GetterError> + From<P2::GetterError>,
        A,
        P2: PartialGetter<I, A>,
    >(
        self,
        other: P2,
    ) -> PartialGetterImpl<S, A, ComposedPartialGetter<Self, P2, E, S, I, A>> {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self,
            other,
            Into::into,
            Into::into,
        ))
    }

    pub fn compose_with_partial_getter_with_mappers<E, A, P2: PartialGetter<I, A>>(
        self,
        other: P2,
        error_mapper1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, ComposedPartialGetter<Self, P2, E, S, I, A>> {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self,
            other,
            error_mapper1,
            error_mapper_2,
        ))
    }

    pub fn compose_with_getter<A, O2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, O2>,
    ) -> PartialGetterImpl<
        S,
        A,
        ComposedPartialGetter<Self, LensImpl<I, A, O2>, P1::GetterError, S, I, A>,
    > {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self, other, identity, infallible,
        ))
    }

    pub fn compose_with_prism<
        E: From<P1::GetterError> + From<P2::GetterError>,
        A,
        P2: PartialGetter<I, A>,
    >(
        self,
        other: P2,
    ) -> PartialGetterImpl<S, A, ComposedPartialGetter<Self, P2, E, S, I, A>> {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self,
            other,
            Into::into,
            Into::into,
        ))
    }

    pub fn compose_with_prism_with_mappers<E, A, P2: PartialGetter<I, A>>(
        self,
        other: P2,
        error_mapper1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, ComposedPartialGetter<Self, P2, E, S, I, A>> {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self,
            other,
            error_mapper1,
            error_mapper_2,
        ))
    }

    pub fn compose_with_lens<A, O2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, O2>,
    ) -> PartialGetterImpl<
        S,
        A,
        ComposedPartialGetter<Self, LensImpl<I, A, O2>, P1::GetterError, S, I, A>,
    > {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self, other, identity, infallible,
        ))
    }

    pub fn compose_with_fallible_iso<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PartialGetterImpl<S, A, ComposedPartialGetter<Self, FallibleIsoImpl<I, A, F2>, E, S, I, A>>
    where
        E: From<F2::GetterError> + From<P1::GetterError>,
    {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self,
            other,
            Into::into,
            Into::into,
        ))
    }

    pub fn compose_with_fallible_iso_with_mappers<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
        getter_error_mapper_1: fn(P1::GetterError) -> E,
        getter_error_mapper_2: fn(F2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, ComposedPartialGetter<Self, FallibleIsoImpl<I, A, F2>, E, S, I, A>>
    {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self,
            other,
            getter_error_mapper_1,
            getter_error_mapper_2,
        ))
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> PartialGetterImpl<
        S,
        A,
        ComposedPartialGetter<Self, IsoImpl<I, A, ISO2>, P1::GetterError, S, I, A>,
    > {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self, other, identity, infallible,
        ))
    }
}
