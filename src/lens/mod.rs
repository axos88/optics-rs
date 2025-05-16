use crate::lens::composed::ComposedLens;
use crate::prism::composed::ComposedPrism;
use crate::prism::{ComposeWithPrism, PrismImpl};
use core::convert::Infallible;
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
pub use composed::new as composed_lens;
pub use mapped::new as mapped_lens;
use crate::getter::Getter;
use crate::setter::Setter;

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
pub trait Lens<S, A>: Getter<S, A> + Setter<S, A> {}

pub trait ComposeWithLens<S, I> {
    type Result<A, L: Lens<I, A>>;
    fn compose_with_lens<A, L: Lens<I, A>>(self, other: LensImpl<I, A, L>) -> Self::Result<A, L>;
}

pub struct LensImpl<S, A, L: Lens<S, A>>(pub L, PhantomData<(S, A)>);

impl<S, A, L: Lens<S, A>> LensImpl<S, A, L> {
    pub fn new(l: L) -> Self {
        LensImpl(l, PhantomData)
    }
}

impl<S, A, L: Lens<S, A>> Getter<S, A> for LensImpl<S, A, L> {
    fn get(&self, source: &S) -> A {
        self.0.get(source)
    }
}

impl<S, A, L: Lens<S, A>> Setter<S, A> for LensImpl<S, A, L> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, L: Lens<S, A>> Lens<S, A> for LensImpl<S, A, L> {}


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

// impl<S, I, L: Lens<S, I>> ComposeWithFallibleIso<S, I> for LensImpl<S, I, L> {
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
// impl<S, I, L: Lens<S, I>> ComposeWithIso<S, I> for LensImpl<S, I, L> {
//     type Result<A, ISO2: Iso<I, A>> =
//         LensImpl<S, A, ComposedLens<Self, IsoImpl<I, A, ISO2>, S, I, A>>;
// 
//     fn compose_with_iso<A, O2: Iso<I, A>>(self, other: IsoImpl<I, A, O2>) -> Self::Result<A, O2> {
//         LensImpl::new(ComposedLens::new(self, other))
//     }
// }
