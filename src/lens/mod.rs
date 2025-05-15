use crate::fallible_iso::{ComposeWithFallibleIso, FallibleIso, FallibleIsoImpl};
use crate::lens::composed::ComposedLens;
use crate::prism::composed::ComposedPrism;
use crate::prism::{ComposeWithPrism, PrismImpl};
use crate::{ComposeWithIso, Iso, IsoImpl, Optic, Prism};
use core::convert::Infallible;
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
pub use composed::new as composed_lens;
pub use mapped::new as mapped_lens;

/// An optic for focusing on a value that is guaranteed to exist within a larger structure.
///
/// A `Lens` is appropriate for product types (e.g., structs) where the focus is always present.
/// Unlike a [`Prism`], a `Lens` cannot fail to retrieve its focus — hence its associated
/// [`Optic::Error`] type is fixed to `Infallible`.
///
/// It can also act as a [`Prism`] for compatibility in compositions.
///
/// # See Also
///
/// - [`Optic`] — base trait implemented by all optics
/// - [`Prism`] — optional focus optic for sum types
/// - [`Iso`] — reversible transformations
/// - [`FallibleIso`] — reversible transformations with fallible forward mapping
pub trait Lens<S, A>: Optic<S, A, Error = Infallible> {
    /// Retrieves the focus value `A` from the source `S`.
    ///
    /// # Arguments
    ///
    /// - `source` — A reference to the source structure containing the focus.
    ///
    /// # Returns
    ///
    /// The focus value `A`.
    ///
    /// # Behavior
    ///
    /// This operation cannot fail and should not mutate the source.
    ///
    /// # Notes
    ///
    /// This is the infallible counterpart of [`Optic::try_get`], provided because for
    /// lenses the focus is always guaranteed to be present.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use std::net::SocketAddrV4;
    /// use optics::{Lens, LensImpl};
    /// let lens = LensImpl::<SocketAddrV4, u16, _>::mapped(|a| a.port(), |a, p| a.set_port(p));
    ///
    /// let addr = SocketAddrV4::new("127.0.0.1".parse().unwrap(), 8080);
    /// let port = lens.get(&addr);
    ///
    /// assert_eq!(port, 8080u16);
    /// ```
    fn get(&self, source: &S) -> A;
}

pub trait ComposeWithLens<S, I> {
    type Result<A, L: Lens<I, A>>;
    fn compose_with_lens<A, L: Lens<I, A>>(self, other: LensImpl<I, A, L>) -> Self::Result<A, L>;
}

/// A trait for composing a `Lens` with other optic types.
///
/// This trait enables the composition of a `Lens` with other types of optics, such as another `Lens`,
/// `Iso`, `FallibleIso`, or `Prism`.
///
/// # Type Parameters
/// - `S`: The source type that the `Lens` operates on.
/// - `I`: The intermediate type in the optic composition.
/// - `A`: The final target type that the optic composition focuses on.
/// - `O2`: The type of the other optic being composed with this `Lens`.
///
/// # Methods
/// The methods in this trait allow composing a `Lens` with various optic types, returning a composed
/// optic that represents a combined focus on the data. Each method corresponds to a different way of
/// composing optics, resulting in a new type of optic
///
/// # See Also
/// - [`Prism`] — the core trait that defines prism-based optics.
/// - [`Lens`] — a trait for working with lens-based optics, a subset of `Optic`.
/// - [`Iso`] — a trait for reversible transformations between types.
/// - [`FallibleIso`] — a trait for isomorphisms that may fail.
/// - [`ComposedLens`] — a type representing the possible result of composing a lens with other optics
/// - [`ComposedPrism`] — a type representing the possible result of composing a lens with other optics
pub trait ComposableLens<S, I, A, O2: Optic<I, A>>: Lens<S, I> + Sized {
    /// Composes the current `Lens` with a `Lens`.
    ///
    /// This method combines a `Lens` with a `Lens`, resulting in a new `Lens`.
    ///
    /// # Requirements
    /// - `other`: The `Lens` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Lens<S, A>`, which is the resulting composed optic.
    // fn compose_lens_with_lens(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    // where
    //   O2: Lens<I, A>;

    /// Composes the current `Lens` with a `Prism`.
    ///
    /// This method combines a `Lens` with a `Prism`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Prism` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    // fn compose_lens_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //   Self: Prism<S, I>,
    //   O2: Prism<I, A>;

    /// Composes the current `Lens` with a `FallibleIso`.
    ///
    /// This method combines a `Lens` with a `FallibleIso`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `FallibleIso` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    // TODO
    // fn compose_lens_with_fallible_iso(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    // where
    //     O2: FallibleIso<I, A>;

    /// Composes the current `Lens` with an `Iso`.
    ///
    /// This method combines a `Lens` with a `Iso`, resulting in a new `Lens`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Lens<S, A>`, which is the resulting composed optic.
    ///
    // TODO
    // fn compose_lens_with_iso(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    // where
    //     O2: Iso<I, A>;
    fn foo() {}
}

pub struct LensImpl<S, A, L: Lens<S, A>>(pub L, PhantomData<(S, A)>);

impl<S, A, L: Lens<S, A>> LensImpl<S, A, L> {
    pub fn new(l: L) -> Self {
        LensImpl(l, PhantomData)
    }
}

impl<S, A, L: Lens<S, A>> Optic<S, A> for LensImpl<S, A, L> {
    type Error = L::Error;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.0.try_get(source)
    }

    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, L: Lens<S, A>> Prism<S, A> for LensImpl<S, A, L> {
    fn preview(&self, source: &S) -> Option<A> {
        Some(self.get(source))
    }
}

impl<S, A, L: Lens<S, A>> Lens<S, A> for LensImpl<S, A, L> {
    fn get(&self, source: &S) -> A {
        self.0.get(source)
    }
}

impl<S, I, L: Lens<S, I>> ComposeWithLens<S, I> for LensImpl<S, I, L> {
    type Result<A, L2: Lens<I, A>> = LensImpl<S, A, ComposedLens<Self, L2, S, I, A>>;

    fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> Self::Result<A, L2> {
        LensImpl::new(ComposedLens::new(self, other.0))
    }
}

impl<S, I, L: Lens<S, I>> ComposeWithPrism<S, I> for LensImpl<S, I, L> {
    type Result<A, P2: Prism<I, A>> = PrismImpl<S, A, ComposedPrism<Self, P2, S, I, A>>;

    fn compose_with_prism<A, P2: Prism<I, A>>(self, other: P2) -> Self::Result<A, P2> {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}

impl<S, I, L: Lens<S, I>> ComposeWithFallibleIso<S, I> for LensImpl<S, I, L> {
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

impl<S, I, L: Lens<S, I>> ComposeWithIso<S, I> for LensImpl<S, I, L> {
    type Result<A, ISO2: Iso<I, A>> =
        LensImpl<S, A, ComposedLens<Self, IsoImpl<I, A, ISO2>, S, I, A>>;

    fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2> {
        LensImpl::new(ComposedLens::new(self, other))
    }
}
