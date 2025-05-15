use crate::fallible_iso::FallibleIso;
use crate::lens::Lens;
use crate::optic::Optic;
use crate::prism::Prism;
use core::convert::Infallible;
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;

use crate::iso::composed::ComposedIso;
use crate::lens::composed::ComposedLens;
use crate::prism::composed::ComposedPrism;
use crate::{
    ComposeWithFallibleIso, ComposeWithLens, ComposeWithPrism, FallibleIsoImpl, LensImpl, PrismImpl,
};
pub use composed::new as composed_iso;
pub use mapped::new as mapped_iso;

/// An isomorphism between two types `S` and `A`.
///
/// An `Iso` is a bidirectional optic that provides a one-to-one, lossless, and reversible mapping
/// between a source type `S` and a focus type `A`. Unlike general `Prism` or `Lens` optics, an
/// `Iso` guarantees that for every `S` there exists exactly one corresponding `A`, and vice versa.
///
/// Because it is total and invertible, an `Iso` is both a `Lens` and a `Prism`, as well as a
/// `FallibleIso` with an infallible error type (`Infallible`). This means it participates fully
/// in the optic hierarchy, providing total reads, total writes, and reversible transformations.
///
/// # Supertraits
/// - [`Optic<S, A, Error = Infallible>`] — ensures that operations on the `Iso` cannot fail.
/// - [`FallibleIso<S, A>`] — provides the fallible isomorphism API, but with `Infallible` error.
/// - [`Prism<S, A>`] — allows using this `Iso` as a `Prism`.
/// - [`Lens<S, A>`] — allows using this `Iso` as a `Lens`.
///
/// # See Also
/// - [`Lens`] — for total, read/write optics.
/// - [`Prism`] — for partial optics.
/// - [`FallibleIso`] — for reversible optics that can fail.
/// - [`Optic`] — the base trait for all optics.
pub trait Iso<S, A>: Optic<S, A, Error = Infallible> {
    /// Performs the reverse transformation from the focus type `A` back to the source type `S`.
    ///
    /// Since an `Iso` guarantees a total, bijective mapping, this method must always succeed.
    ///
    /// # Arguments
    /// * `source` — A reference to the focus type value `A`.
    ///
    /// # Returns
    /// The corresponding source type value `S`.
    fn reverse_get(&self, value: &A) -> S;
}

pub trait ComposeWithIso<S, I> {
    type Result<A, O2: Iso<I, A>>;
    fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2>;
}

pub struct IsoImpl<S, A, ISO: Iso<S, A>>(pub ISO, PhantomData<(S, A)>);

impl<S, A, ISO: Iso<S, A>> IsoImpl<S, A, ISO> {
    pub fn new(i: ISO) -> Self {
        IsoImpl(i, PhantomData)
    }
}

impl<S, A, ISO: Iso<S, A>> Optic<S, A> for IsoImpl<S, A, ISO> {
    type Error = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.0.try_get(source)
    }

    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, ISO: Iso<S, A>> Prism<S, A> for IsoImpl<S, A, ISO> {
    fn preview(&self, source: &S) -> Option<A> {
        Some(self.get(source))
    }
}

impl<S, A, ISO: Iso<S, A>> Lens<S, A> for IsoImpl<S, A, ISO> {
    fn get(&self, source: &S) -> A {
        let Ok(a) = self.0.try_get(source);
        a
    }
}

impl<S, A, ISO: Iso<S, A>> FallibleIso<S, A> for IsoImpl<S, A, ISO> {
    fn try_reverse_get(&self, value: &A) -> Result<S, Self::Error> {
        Ok(self.0.reverse_get(value))
    }
}

impl<S, A, ISO: Iso<S, A>> Iso<S, A> for IsoImpl<S, A, ISO> {
    fn reverse_get(&self, value: &A) -> S {
        self.0.reverse_get(value)
    }
}

impl<S, I, ISO: Iso<S, I>> ComposeWithLens<S, I> for IsoImpl<S, I, ISO> {
    type Result<A, L2: Lens<I, A>> =
        LensImpl<S, A, ComposedLens<Self, LensImpl<I, A, L2>, S, I, A>>;

    fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> Self::Result<A, L2> {
        LensImpl::new(ComposedLens::new(self, other))
    }
}

impl<S, I, ISO: Iso<S, I>> ComposeWithPrism<S, I> for IsoImpl<S, I, ISO> {
    type Result<A, P2: Prism<I, A>> = PrismImpl<S, A, ComposedPrism<Self, P2, S, I, A>>;

    fn compose_with_prism<A, P2: Prism<I, A>>(self, other: P2) -> Self::Result<A, P2> {
        PrismImpl::new(ComposedPrism::new(self, other))
    }
}

impl<S, I, ISO: Iso<S, I>> ComposeWithFallibleIso<S, I> for IsoImpl<S, I, ISO> {
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

impl<S, I, ISO: Iso<S, I>> ComposeWithIso<S, I> for IsoImpl<S, I, ISO> {
    type Result<A, ISO2: Iso<I, A>> = IsoImpl<S, A, ComposedIso<Self, ISO2, S, I, A>>;

    fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2> {
        IsoImpl::new(ComposedIso::new(self, other.0))
    }
}
