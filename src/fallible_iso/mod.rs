use crate::lens::Lens;
use crate::prism::Prism;
use crate::{
    HasPartialGetter, HasSetter, Iso, IsoImpl, LensImpl, PartialGetter, PrismImpl, Setter,
    infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;

use crate::HasPartialReversible;
use crate::fallible_iso::composed::ComposedFallibleIso;
use crate::prism::composed::ComposedPrism;
pub use composed::new as composed_fallible_iso;
pub use mapped::new as mapped_fallible_iso;

/// A bidirectional, fallible isomorphism between two types `S` and `A`.
///
/// A `FallibleIso` is an optic that provides a potentially lossy, reversible mapping between a
/// source type `S` and a focus type `A`, where **both** the forward (`S → A`) and reverse
/// (`A → S`) transformations can fail independently.
///
/// This makes it suitable for conversions where neither direction is guaranteed to succeed in
/// all cases. Examples include parsing, type coercion, or partial decoding tasks where values
/// may not always be representable in the other form.
///
/// # Supertraits
/// - [`Optic<S, A>`] — provides the primary optic interface for fallible `get` and `set` operations.
/// - [`Prism<S, A>`] — allows using this `FallibleIso` as a `Prism`.
///
/// # Error Semantics
/// The associated `Error` type on the `Optic` supertrait defines the possible error value for
/// both the `try_get` and `try_reverse_get` operations.
///
/// # See Also
/// - [`Iso`] — for total, infallible isomorphisms.
/// - [`Prism`] — for partial optics where only one direction may be partial.
/// - [`Optic`] — the base trait for all optics.
pub trait FallibleIso<S, A>:
    HasPartialGetter<S, A> + HasSetter<S, A> + HasPartialReversible<S, A>
{
}

pub struct FallibleIsoImpl<S, A, F: FallibleIso<S, A>>(pub F, PhantomData<(S, A)>);

impl<S, A, F: FallibleIso<S, A>> FallibleIsoImpl<S, A, F> {
    pub fn new(l: F) -> Self {
        FallibleIsoImpl(l, PhantomData)
    }
}

impl<S, A, F: FallibleIso<S, A>> HasPartialGetter<S, A> for FallibleIsoImpl<S, A, F> {
    type GetterError = F::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, A, F: FallibleIso<S, A>> HasSetter<S, A> for FallibleIsoImpl<S, A, F> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, F: FallibleIso<S, A>> HasPartialReversible<S, A> for FallibleIsoImpl<S, A, F> {
    type ReverseError = F::ReverseError;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        self.0.try_reverse_get(value)
    }
}

impl<S, A, F: FallibleIso<S, A>> PartialGetter<S, A> for FallibleIsoImpl<S, A, F> {}
impl<S, A, F: FallibleIso<S, A>> Setter<S, A> for FallibleIsoImpl<S, A, F> {}
impl<S, A, F: FallibleIso<S, A>> Prism<S, A> for FallibleIsoImpl<S, A, F> {}
impl<S, A, F: FallibleIso<S, A>> FallibleIso<S, A> for FallibleIsoImpl<S, A, F> {}

impl<S, I, F1: FallibleIso<S, I>> FallibleIsoImpl<S, I, F1> {
    pub fn compose_with_prism<E, A, P2: Prism<I, A>>(
        self,
        other: P2,
    ) -> PrismImpl<S, A, ComposedPrism<Self, P2, E, S, I, A>>
    where
        E: From<F1::GetterError> + From<P2::GetterError>,
    {
        PrismImpl::new(ComposedPrism::new(self, other, Into::into, Into::into))
    }

    pub fn compose_with_prism_with_mappers<E, A, P2: Prism<I, A>>(
        self,
        other: P2,
        error_mapper_1: fn(F1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PrismImpl<S, A, ComposedPrism<Self, P2, E, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(
            self,
            other,
            error_mapper_1,
            error_mapper_2,
        ))
    }

    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> PrismImpl<S, A, ComposedPrism<Self, LensImpl<I, A, L2>, F1::GetterError, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(self, other, identity, infallible))
    }

    pub fn compose_with_fallible_iso<GE, RE, A, F2: FallibleIso<I, A>>(
        self,
        other: F2,
    ) -> FallibleIsoImpl<S, A, ComposedFallibleIso<Self, F2, GE, RE, S, I, A>>
    where
        GE: From<F1::GetterError> + From<F2::GetterError>,
        RE: From<F1::ReverseError> + From<F2::ReverseError>,
    {
        FallibleIsoImpl::new(ComposedFallibleIso::new(
            self,
            other,
            Into::into,
            Into::into,
            Into::into,
            Into::into,
        ))
    }

    pub fn compose_with_fallible_iso_with_mappers<GE, RE, A, F2: FallibleIso<I, A>>(
        self,
        other: F2,
        getter_error_mapper_1: fn(F1::GetterError) -> GE,
        getter_error_mapper_2: fn(F2::GetterError) -> GE,
        reverse_error_mapper_1: fn(F1::ReverseError) -> RE,
        reverse_error_mapper_2: fn(F2::ReverseError) -> RE,
    ) -> FallibleIsoImpl<S, A, ComposedFallibleIso<Self, F2, GE, RE, S, I, A>> {
        FallibleIsoImpl::new(ComposedFallibleIso::new(
            self,
            other,
            getter_error_mapper_1,
            getter_error_mapper_2,
            reverse_error_mapper_1,
            reverse_error_mapper_2,
        ))
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> FallibleIsoImpl<
        S,
        A,
        ComposedFallibleIso<Self, IsoImpl<I, A, ISO2>, F1::GetterError, F1::ReverseError, S, I, A>,
    > {
        FallibleIsoImpl::new(ComposedFallibleIso::new(
            self, other, identity, infallible, identity, infallible,
        ))
    }
}

// impl<S, I, F: FallibleIso<S, I>> ComposeWithIso<S, I> for FallibleIsoImpl<S, I, F>
// where
//     F::Error: From<Infallible>,
// {
//     type Result<A, ISO2: Iso<I, A>> =
//         FallibleIsoImpl<S, A, ComposedFallibleIso<Self, IsoImpl<I, A, ISO2>, F::Error, S, I, A>>;
//
//     fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2> {
//         FallibleIsoImpl::new(ComposedFallibleIso::new(self, other))
//     }
// }
