use core::marker::PhantomData;
use crate::partial_getter::PartialGetter;
use crate::setter::Setter;
use crate::prism::composed::ComposedPrism;
pub use composed::new as composed_prism;
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

pub trait ComposeWithPrism<S, I>: PartialGetter<S, I> + Setter<S, I> {
    type Result<E, A, P: Prism<I, A>>;
    fn compose_with_prism<E, A, P: Prism<I, A>>(self, other: P, error_mapper_1: fn(Self::GetterError) -> E, error_mapper_2: fn(P::GetterError) -> E) -> Self::Result<E, A, P>;
}

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

impl<S, I, P: Prism<S, I>> ComposeWithPrism<S, I> for PrismImpl<S, I, P> {
    type Result<E, A, P2: Prism<I, A>> = PrismImpl<S, A, ComposedPrism<Self, P2, E, S, I, A>>;

    fn compose_with_prism<E, A, P2: Prism<I, A>>(self, other: P2, error_mapper1: fn(P::GetterError) -> E, error_mapper_2: fn(P2::GetterError) -> E) -> Self::Result<E, A, P2> {
        PrismImpl::new(ComposedPrism::new(self, other, error_mapper1, error_mapper_2))
    }
}

// impl<S, I, P: Prism<S, I>> ComposeWithLens<S, I> for PrismImpl<S, I, P> {
//     type Result<A, L2: Lens<I, A>> =
//         PrismImpl<S, A, ComposedPrism<Self, LensImpl<I, A, L2>, S, I, A>>;
//
//     fn compose_with_lens<A, L: Lens<I, A>>(self, other: LensImpl<I, A, L>) -> Self::Result<A, L> {
//         PrismImpl::new(ComposedPrism::new(self, other))
//     }
// }
//
// impl<S, I, P: Prism<S, I>> ComposeWithFallibleIso<S, I> for PrismImpl<S, I, P> {
//     type Result<E, A, F2: FallibleIso<I, A>>
//         = PrismImpl<S, A, ComposedPrism<Self, FallibleIsoImpl<I, A, F2>, S, I, A>>
//     where
//         Self::Error: Into<E>,
//         F2::Error: Into<E>;
//
//     fn compose_with_fallible_iso<E, A, F: FallibleIso<I, A>>(
//         self,
//         other: FallibleIsoImpl<I, A, F>,
//     ) -> Self::Result<E, A, F>
//     where
//         Self::Error: Into<E>,
//         F::Error: Into<E>,
//     {
//         PrismImpl::new(ComposedPrism::new(self, other))
//     }
// }
//
// impl<S, I, P: Prism<S, I>> ComposeWithIso<S, I> for PrismImpl<S, I, P> {
//     type Result<A, ISO2: Iso<I, A>> =
//         PrismImpl<S, A, ComposedPrism<Self, IsoImpl<I, A, ISO2>, S, I, A>>;
//
//     fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2> {
//         PrismImpl::new(ComposedPrism::new(self, other))
//     }
// }
