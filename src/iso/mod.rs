use crate::fallible_iso::FallibleIso;
use crate::iso::composed::ComposedIso;
use crate::lens::Lens;
use crate::lens::composed::ComposedLens;
use crate::prism::Prism;
use crate::prism::composed::ComposedPrism;
use crate::{
    FallibleIsoImpl, Getter, HasGetter, HasPartialGetter, HasPartialReversible, HasReversible,
    HasSetter, LensImpl, PartialGetter, PrismImpl, Setter, infallible,
};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
use crate::fallible_iso::composed::ComposedFallibleIso;
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
pub trait Iso<S, A>: HasGetter<S, A> + HasSetter<S, A> + HasReversible<S, A> {}

pub struct IsoImpl<S, A, ISO: Iso<S, A>>(pub ISO, PhantomData<(S, A)>);

impl<S, A, ISO: Iso<S, A>> IsoImpl<S, A, ISO> {
    pub fn new(i: ISO) -> Self {
        IsoImpl(i, PhantomData)
    }
}

impl<S, A, ISO: Iso<S, A>> HasPartialGetter<S, A> for IsoImpl<S, A, ISO> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, A, ISO: Iso<S, A>> HasGetter<S, A> for IsoImpl<S, A, ISO> {
    fn get(&self, source: &S) -> A {
        self.0.get(source)
    }
}

impl<S, A, ISO: Iso<S, A>> HasSetter<S, A> for IsoImpl<S, A, ISO> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, ISO: Iso<S, A>> HasPartialReversible<S, A> for IsoImpl<S, A, ISO> {
    type ReverseError = Infallible;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        Ok(self.0.reverse_get(value))
    }
}

impl<S, A, ISO: Iso<S, A>> HasReversible<S, A> for IsoImpl<S, A, ISO> {
    fn reverse_get(&self, value: &A) -> S {
        self.0.reverse_get(value)
    }
}

impl<S, A, ISO: Iso<S, A>> PartialGetter<S, A> for IsoImpl<S, A, ISO> {}
impl<S, A, ISO: Iso<S, A>> Getter<S, A> for IsoImpl<S, A, ISO> {}
impl<S, A, ISO: Iso<S, A>> Setter<S, A> for IsoImpl<S, A, ISO> {}
impl<S, A, ISO: Iso<S, A>> Lens<S, A> for IsoImpl<S, A, ISO> {}
impl<S, A, ISO: Iso<S, A>> Prism<S, A> for IsoImpl<S, A, ISO> {}
impl<S, A, ISO: Iso<S, A>> FallibleIso<S, A> for IsoImpl<S, A, ISO> {}
impl<S, A, ISO: Iso<S, A>> Iso<S, A> for IsoImpl<S, A, ISO> {}

impl<S, I, ISO1: Iso<S, I>> IsoImpl<S, I, ISO1> {
    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> LensImpl<S, A, ComposedLens<Self, LensImpl<I, A, L2>, S, I, A>> {
        LensImpl::new(ComposedLens::new(self, other))
    }

    pub fn compose_with_prism<A, P2: Prism<I, A>>(
        self,
        other: P2,
    ) -> PrismImpl<S, A, ComposedPrism<Self, P2, P2::GetterError, S, I, A>> {
        PrismImpl::new(ComposedPrism::new(self, other, infallible, identity))
    }

    pub fn compose_with_fallible_iso<A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> FallibleIsoImpl<
        S,
        A,
        ComposedFallibleIso<
            Self,
            FallibleIsoImpl<I, A, F2>,
            F2::GetterError,
            F2::ReverseError,
            S,
            I,
            A,
        >,
    > {
        FallibleIsoImpl::new(ComposedFallibleIso::new(
            self, other, infallible, identity, infallible, identity,
        ))
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> IsoImpl<S, A, ComposedIso<Self, ISO2, S, I, A>> {
        IsoImpl::new(ComposedIso::new(self, other.0))
    }
}
