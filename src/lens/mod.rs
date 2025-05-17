use crate::lens::composed::ComposedLens;
use crate::prism::composed::ComposedPrism;
use crate::prism::{Prism, PrismImpl};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
use crate::fallible_iso::{FallibleIso, FallibleIsoImpl};
use crate::{Getter, HasGetter};
use crate::{HasPartialGetter, PartialGetter};
use crate::{HasSetter, Setter};
use crate::{Iso, IsoImpl, infallible};
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
pub trait Lens<S, A>: HasGetter<S, A> + HasSetter<S, A> {}

pub struct LensImpl<S, A, L: Lens<S, A>>(pub L, PhantomData<(S, A)>);

impl<S, A, L: Lens<S, A>> LensImpl<S, A, L> {
    pub fn new(l: L) -> Self {
        LensImpl(l, PhantomData)
    }
}

impl<S, A, L: Lens<S, A>> HasGetter<S, A> for LensImpl<S, A, L> {
    fn get(&self, source: &S) -> A {
        self.0.get(source)
    }
}

impl<S, A, L: Lens<S, A>> HasPartialGetter<S, A> for LensImpl<S, A, L> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, A, L: Lens<S, A>> HasSetter<S, A> for LensImpl<S, A, L> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, L: Lens<S, A>> Getter<S, A> for LensImpl<S, A, L> {}
impl<S, A, L: Lens<S, A>> PartialGetter<S, A> for LensImpl<S, A, L> {}
impl<S, A, L: Lens<S, A>> Setter<S, A> for LensImpl<S, A, L> {}
impl<S, A, L: Lens<S, A>> Lens<S, A> for LensImpl<S, A, L> {}
impl<S, A, L: Lens<S, A>> Prism<S, A> for LensImpl<S, A, L> {}

impl<S, I, L: Lens<S, I>> LensImpl<S, I, L> {
    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> LensImpl<S, A, ComposedLens<Self, L2, S, I, A>> {
        LensImpl::new(ComposedLens::new(self, other.0))
    }

    pub fn compose_with_prism<A, P: Prism<I, A>>(
        self,
        other: P,
    ) -> PrismImpl<S, A, ComposedPrism<Self, P, P::GetterError, S, I, A>> {
        ComposedPrism::new(self, other, |e| match e {}, identity).wrap()
    }

    pub fn compose_with_fallible_iso<A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PrismImpl<S, A, ComposedPrism<Self, FallibleIsoImpl<I, A, F2>, F2::GetterError, S, I, A>>
    {
        ComposedPrism::new(self, other, infallible, identity).wrap()
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> LensImpl<S, A, ComposedLens<Self, IsoImpl<I, A, ISO2>, S, I, A>> {
        LensImpl::new(ComposedLens::new(self, other))
    }
}

pub fn identity_lens<S: Clone, E> () -> LensImpl<S, S, impl Lens<S, S>> {
    mapped_lens(|x: &S| x.clone(), |_, _| ())
}