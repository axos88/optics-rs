use crate::lens::{ComposeWithLens, Lens, LensImpl};
use crate::{ComposeWithIso, Iso, IsoImpl, Optic};
use core::convert::Infallible;
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
use crate::fallible_iso::{ComposeWithFallibleIso, FallibleIso, FallibleIsoImpl};
use crate::prism::composed::ComposedPrism;
pub use composed::new as composed_prism;
pub use mapped::new as mapped_prism;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// An error type indicating that a `Prism` failed to extract a focus value.
///
/// A `Prism` represents an optional focus within a sum type, and attempting to
/// access a value using its `preview` or similar method may fail if the underlying
/// value is in a different variant or does not contain the expected focus.
///
/// `NoFocus` serves as a simple, lightweight error type to signal this situation.
pub struct NoFocus;

impl From<Infallible> for NoFocus {
    fn from(_value: Infallible) -> Self {
        NoFocus
    }
}

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
pub trait Prism<S, A>: Optic<S, A> {
    /// Attempt to extract a value of type `A` from `S`
    fn preview(&self, source: &S) -> Option<A>;
}

pub trait ComposeWithPrism<S, I> {
    type Result<A, P: Prism<I, A>>: Optic<S, A>;
    fn compose_with_prism<A, P: Prism<I, A>>(self, other: P) -> Self::Result<A, P>;
}

/// A trait for composing a `Prism` with other optic types.
///
/// This trait enables the composition of a `Prism` with other types of optics, such as `Lens`,
/// `Iso`, `FallibleIso`, and another `Prism`.
///
/// # Type Parameters
/// - `S`: The source type that the `Prism` operates on.
/// - `I`: The intermediate type in the optic composition.
/// - `A`: The final target type that the optic composition focuses on.
/// - `O2`: The type of the other optic being composed with this `Prism`.
///
/// # Methods
/// The methods in this trait allow composing a `Prism` with various optic types, returning a composed
/// optic that represents a combined focus on the data. Each method corresponds to a different way of
/// composing optics, resulting in a new type of optic
///
/// # See Also
/// - [`Prism`] — the core trait that defines prism-based optics.
/// - [`Lens`] — a trait for working with lens-based optics, a subset of `Optic`.
/// - [`Iso`] — a trait for reversible transformations between types.
/// - [`FallibleIso`] — a trait for isomorphisms that may fail.
/// - [`ComposedPrism`] — a type representing the possible result of composing a lens with other optics.
pub trait ComposablePrism<S, I, A, O2: Optic<I, A>>: Prism<S, I> + Sized {
    /// Composes the current `Prism` with a `Lens`.
    ///
    /// This method combines a `Prism` with a `Lens`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Lens` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    // fn compose_prism_with_lens(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //   O2: Prism<I, A>;

    /// Composes the current `Prism` with a `Prism`.
    ///
    /// This method combines a `Prism` with a `Prism`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Prism` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    // fn compose_prism_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //   O2: Prism<I, A>;

    /// Composes the current `Prism` with a `FallibleIso`.
    ///
    /// This method combines a `Prism` with a `FallibleIso`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `FallibleIso` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    // TODO
    // fn compose_prism_with_fallible_iso<E>(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //     O2: FallibleIso<I, A>,
    //     E: From<Self::Error> + From<O2::Error>;

    /// Composes the current `Prism` with an `Iso`.
    ///
    /// This method combines a `Prism` with an `Iso`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    // TODO
    // fn compose_prism_with_iso(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //     O2: Iso<I, A>;
    fn foo() {}
}

pub struct PrismImpl<S, A, P: Prism<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, P: Prism<S, A>> PrismImpl<S, A, P> {
    pub fn new(prism: P) -> Self {
        PrismImpl(prism, PhantomData)
    }
}

impl<S, A, P: Prism<S, A>> Optic<S, A> for PrismImpl<S, A, P> {
    type Error = P::Error;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.0.try_get(source)
    }

    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, P: Prism<S, A>> Prism<S, A> for PrismImpl<S, A, P> {
    fn preview(&self, source: &S) -> Option<A> {
        self.0.preview(source)
    }
}

impl<S, I, P: Prism<S, I>> ComposeWithPrism<S, I> for PrismImpl<S, I, P> {
    type Result<A, P2: Prism<I, A>> = PrismImpl<S, A, ComposedPrism<Self, P2, S, I, A>>;

    fn compose_with_prism<A, P2: Prism<I, A>>(self, other: P2) -> Self::Result<A, P2> {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}

impl<S, I, P: Prism<S, I>> ComposeWithLens<S, I> for PrismImpl<S, I, P> {
    type Result<A, L2: Lens<I, A>> =
        PrismImpl<S, A, ComposedPrism<Self, LensImpl<I, A, L2>, S, I, A>>;

    fn compose_with_lens<A, L: Lens<I, A>>(self, other: LensImpl<I, A, L>) -> Self::Result<A, L> {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}

impl<S, I, P: Prism<S, I>> ComposeWithFallibleIso<S, I> for PrismImpl<S, I, P> {
    type Result<E, A, F2: FallibleIso<I, A>>
        = PrismImpl<S, A, ComposedPrism<Self, FallibleIsoImpl<I, A, F2>, S, I, A>>
    where
        Self::Error: Into<E>,
        F2::Error: Into<E>;

    fn compose_with_fallible_iso<E, A, F: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F>,
    ) -> Self::Result<E, A, F>
    where
        Self::Error: Into<E>,
        F::Error: Into<E>,
    {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}

impl<S, I, P: Prism<S, I>> ComposeWithIso<S, I> for PrismImpl<S, I, P> {
    type Result<A, ISO2: Iso<I, A>> =
        PrismImpl<S, A, ComposedPrism<Self, IsoImpl<I, A, ISO2>, S, I, A>>;

    fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2> {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}
