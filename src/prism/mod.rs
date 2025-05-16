use crate::fallible_iso::{FallibleIso, FallibleIsoImpl};
use crate::lens::{Lens, LensImpl};
use crate::partial_getter::PartialGetter;
use crate::prism::composed::ComposedPrism;
use crate::setter::Setter;
use crate::{Iso, IsoImpl, infallible};
pub use composed::new as composed_prism;
use core::convert::identity;
use core::marker::PhantomData;
pub use mapped::new as mapped_prism;

pub(crate) mod composed;
pub(crate) mod mapped;

/// A `Prism` is an optic that focuses on a potential value inside a sum type.
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
/// `Prism` is currently the most general form of a *concrete optic* in this library.
/// In the future, it may be generalized further to allow any error type in place
/// of the fixed `NoFocus` error. When that happens, `Prism<S, A>` will become a
/// special case of a fully general `Optic<S, A>` abstraction, making this trait
/// redundant and subject to removal in favor of the unified `Optic<S, A>` design.
///
/// This would allow error type specialization while preserving the same core behavior.
///
/// # See Also
/// - [`Optic`] — the base trait for all optic types, potentially unifying `Lens`, `Prism`, and `Iso`
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the forward mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible one-to-one transformation between two types
///
/// - [`NoFocus`] — the current error type returned by `Prism::preview` on failure
pub trait Prism<S, A>: PartialGetter<S, A> + Setter<S, A> {}

pub struct PrismImpl<S, A, P: Prism<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, P: Prism<S, A>> PrismImpl<S, A, P> {
    pub fn new(prism: P) -> Self {
        PrismImpl(prism, PhantomData)
    }
}

impl<S, A, P: Prism<S, A>> PartialGetter<S, A> for PrismImpl<S, A, P> {
    type GetterError = P::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, A, P: Prism<S, A>> Setter<S, A> for PrismImpl<S, A, P> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, P: Prism<S, A>> Prism<S, A> for PrismImpl<S, A, P> {}

impl<S, I, P1: Prism<S, I>> PrismImpl<S, I, P1> {
    pub fn compose_with_prism<
        E: From<P1::GetterError> + From<P2::GetterError>,
        A,
        P2: Prism<I, A>,
    >(
        self,
        other: P2,
    ) -> PrismImpl<S, A, ComposedPrism<Self, P2, E, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(self, other, Into::into, Into::into))
    }

    pub fn compose_with_prism_with_mappers<E, A, P2: Prism<I, A>>(
        self,
        other: P2,
        error_mapper1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PrismImpl<S, A, ComposedPrism<Self, P2, E, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(
            self,
            other,
            error_mapper1,
            error_mapper_2,
        ))
    }

    pub fn compose_with_lens<A, O2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, O2>,
    ) -> PrismImpl<S, A, ComposedPrism<Self, LensImpl<I, A, O2>, P1::GetterError, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(self, other, identity, infallible))
    }

    pub fn compose_with_fallible_iso<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PrismImpl<S, A, ComposedPrism<Self, FallibleIsoImpl<I, A, F2>, E, S, I, A>>
    where
        E: From<F2::GetterError> + From<P1::GetterError>,
    {
        PrismImpl::new(ComposedPrism::new(self, other, Into::into, Into::into))
    }

    pub fn compose_with_fallible_iso_with_mappers<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
        getter_error_mapper_1: fn(P1::GetterError) -> E,
        getter_error_mapper_2: fn(F2::GetterError) -> E,
    ) -> PrismImpl<S, A, ComposedPrism<Self, FallibleIsoImpl<I, A, F2>, E, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(
            self,
            other,
            getter_error_mapper_1,
            getter_error_mapper_2,
        ))
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> PrismImpl<S, A, ComposedPrism<Self, IsoImpl<I, A, ISO2>, P1::GetterError, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(self, other, identity, infallible))
    }
}
