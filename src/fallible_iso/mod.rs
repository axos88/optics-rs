use crate::lens::Lens;
use crate::prism::Prism;
use crate::{
    ComposeWithIso, ComposeWithLens, ComposeWithPrism, Iso, IsoImpl, LensImpl, Optic, PrismImpl,
};
use core::convert::Infallible;
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;

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
pub trait FallibleIso<S, A>: Optic<S, A> {
    /// Attempts to perform the reverse transformation from the focus type `A` back to the source type `S`.
    ///
    /// Since this is a *fallible* isomorphism, the operation may fail if the provided `A` value
    /// cannot be converted back into a valid `S`. The error type is defined by the `Error`
    /// associated type of the [`Optic`] supertrait.
    ///
    /// # Arguments
    /// * `source` — A reference to the focus type value `A`.
    ///
    /// # Returns
    /// `Ok(S)` if the reverse transformation succeeds,
    ///
    /// # Errors
    /// Returns `Err(Self::Error)` if the transformation fails.
    ///
    fn try_reverse_get(&self, value: &A) -> Result<S, Self::Error>;
}

pub trait ComposeWithFallibleIso<S, I>: Optic<S, I> {
    type Result<E, A, F2: FallibleIso<I, A>>
    where
        Self::Error: Into<E>,
        F2::Error: Into<E>;

    fn compose_with_fallible_iso<E, A, F: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F>,
    ) -> Self::Result<E, A, F>
    where
        Self::Error: Into<E>,
        F::Error: Into<E>;
}

/// A trait for composing a `FallibleIso` with other optic types.
///
/// This trait enables the composition of a `FallibleIso` with other types of optics, such as a `Lens`,
/// `Iso`, another `FallibleIso`, or `Prism`.
///
/// # Type Parameters
/// - `S`: The source type that the `FallibleIso` operates on.
/// - `I`: The intermediate type in the optic composition.
/// - `A`: The final target type that the optic composition focuses on.
/// - `O2`: The type of the other optic being composed with this `FallibleIso`.
///
/// # Methods
/// The methods in this trait allow composing a `FallibleIso` with various optic types, returning a composed
/// optic that represents a combined focus on the data. Each method corresponds to a different way of
/// composing optics, resulting in a new type of optic
///
/// # See Also
/// - [`Prism`] — the core trait that defines prism-based optics.
/// - [`Lens`] — a trait for working with lens-based optics, a subset of `Optic`.
/// - [`Iso`] — a trait for reversible transformations between types.
/// - [`FallibleIso`] — a trait for isomorphisms that may fail.
/// - [`crate::ComposedLens`] — a type representing the possible result of composing a lens with other optics
/// - [`ComposedPrism`] — a type representing the possible result of composing a lens with other optics
pub trait ComposableFallibleIso<S, I, A, O2: Optic<I, A>>: FallibleIso<S, I> {
    /// Composes the current `FallibleIso` with an `Lens`.
    ///
    /// This method combines a `FallibleIso` with a `Lens`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Lens` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    // fn compose_fallible_iso_with_lens(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //     O2: Lens<I, A>;

    /// Composes the current `FallibleIso` with an `Prism`.
    ///
    /// This method combines a `FallibleIso` with a `Prism`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Prism` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    // fn compose_fallible_iso_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //     O2: Prism<I, A>;

    /// Composes the current `FallibleIso` with an `FallibleIso`.
    ///
    /// This method combines a `FallibleIso` with a `FallibleIso`, resulting in a new `FallibleIso`.
    ///
    /// # Requirements
    /// - `other`: The `FallibleIso` to compose with the current `FallibleIso`.
    ///
    /// # Returns
    /// - A `FallibleIso<S, A>`, which is the resulting composed optic.
    // fn compose_fallible_iso_with_fallible_iso<E>(
    //     self,
    //     other: O2,
    // ) -> ComposedFallibleIso<Self, O2, E, S, I, A>
    // where
    //     O2: FallibleIso<I, A>,
    //     O2::Error: Into<E>;

    /// Composes the current `FallibleIso` with an `Iso`.
    ///
    /// This method combines a `FallibleIso` with a `Iso`, resulting in a new `FallibleIso`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `FallibleIso<S, A>`, which is the resulting composed optic.
    // fn compose_fallible_iso_with_iso(
    //     self,
    //     other: O2,
    // ) -> ComposedFallibleIso<Self, O2, Self::Error, S, I, A>
    // where
    //     O2: Iso<I, A>;

    fn foo() {}
}

pub struct FallibleIsoImpl<S, A, F: FallibleIso<S, A>>(pub F, PhantomData<(S, A)>);

impl<S, A, F: FallibleIso<S, A>> FallibleIsoImpl<S, A, F> {
    pub fn new(l: F) -> Self {
        FallibleIsoImpl(l, PhantomData)
    }
}

impl<S, A, F: FallibleIso<S, A>> Optic<S, A> for FallibleIsoImpl<S, A, F> {
    type Error = F::Error;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.0.try_get(source)
    }

    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, F: FallibleIso<S, A>> Prism<S, A> for FallibleIsoImpl<S, A, F> {
    fn preview(&self, source: &S) -> Option<A> {
        self.0.try_get(source).ok()
    }
}

impl<S, A, F: FallibleIso<S, A>> FallibleIso<S, A> for FallibleIsoImpl<S, A, F> {
    fn try_reverse_get(&self, source: &A) -> Result<S, Self::Error> {
        self.0.try_reverse_get(source)
    }
}

impl<S, I, F: FallibleIso<S, I>> ComposeWithLens<S, I> for FallibleIsoImpl<S, I, F> {
    type Result<A, L2: Lens<I, A>> =
        PrismImpl<S, A, ComposedPrism<Self, LensImpl<I, A, L2>, S, I, A>>;

    fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> Self::Result<A, L2> {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}

impl<S, I, F: FallibleIso<S, I>> ComposeWithPrism<S, I> for FallibleIsoImpl<S, I, F> {
    type Result<A, P2: Prism<I, A>> = PrismImpl<S, A, ComposedPrism<Self, P2, S, I, A>>;

    fn compose_with_prism<A, P2: Prism<I, A>>(self, other: P2) -> Self::Result<A, P2> {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}

impl<S, I, F: FallibleIso<S, I>> ComposeWithFallibleIso<S, I> for FallibleIsoImpl<S, I, F> {
    type Result<E, A, F2: FallibleIso<I, A>>
        = FallibleIsoImpl<S, A, ComposedFallibleIso<Self, FallibleIsoImpl<I, A, F2>, E, S, I, A>>
    where
        Self::Error: Into<E>,
        F2::Error: Into<E>;

    fn compose_with_fallible_iso<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> Self::Result<E, A, F2>
    where
        Self::Error: Into<E>,
        F2::Error: Into<E>,
    {
        FallibleIsoImpl::new(ComposedFallibleIso::new(self, other))
    }
}

impl<S, I, F: FallibleIso<S, I>> ComposeWithIso<S, I> for FallibleIsoImpl<S, I, F>
where
    F::Error: From<Infallible>,
{
    type Result<A, ISO2: Iso<I, A>> =
        FallibleIsoImpl<S, A, ComposedFallibleIso<Self, IsoImpl<I, A, ISO2>, F::Error, S, I, A>>;

    fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2> {
        FallibleIsoImpl::new(ComposedFallibleIso::new(self, other))
    }
}
