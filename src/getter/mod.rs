use crate::fallible_iso::{FallibleIso, FallibleIsoImpl};
use crate::getter::composed::ComposedGetter;
use crate::lens::{Lens, LensImpl};
use crate::partial_getter::composed::ComposedPartialGetter;
use crate::{mapped_iso, HasGetter, HasPartialGetter, Prism};
use crate::{Iso, IsoImpl, infallible};
use crate::{PartialGetter, PartialGetterImpl, PrismImpl};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
pub use composed::new as composed_getter;
pub use mapped::new as mapped_getter;

/// A `Getter` is an optic that focuses on a potential value inside a sum type.
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
/// `Getter` is currently the most general form of a *concrete optic* in this library.
/// In the future, it may be generalized further to allow any error type in place
/// of the fixed `NoFocus` error. When that happens, `Getter<S, A>` will become a
/// special case of a fully general `Optic<S, A>` abstraction, making this trait
/// redundant and subject to removal in favor of the unified `Optic<S, A>` design.
///
/// This would allow error type specialization while preserving the same core behavior.
///
/// # See Also
/// - [`Optic`] — the base trait for all optic types, potentially unifying `Lens`, `Getter`, and `Iso`
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the forward mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible one-to-one transformation between two types
///
/// - [`NoFocus`] — the current error type returned by `Getter::preview` on failure
pub trait Getter<S, A>: HasGetter<S, A> {}

pub struct GetterImpl<S, A, P: Getter<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, P: Getter<S, A>> GetterImpl<S, A, P> {
    pub fn new(prism: P) -> Self {
        GetterImpl(prism, PhantomData)
    }
}

impl<S, A, P: Getter<S, A>> HasPartialGetter<S, A> for GetterImpl<S, A, P> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, A, P: Getter<S, A>> HasGetter<S, A> for GetterImpl<S, A, P> {
    fn get(&self, source: &S) -> A {
        self.0.get(source)
    }
}

impl<S, A, P: Getter<S, A>> PartialGetter<S, A> for GetterImpl<S, A, P> {}
impl<S, A, P: Getter<S, A>> Getter<S, A> for GetterImpl<S, A, P> {}

impl<S, I, G1: Getter<S, I>> GetterImpl<S, I, G1> {
    pub fn compose_with_getter<A, G2: Getter<I, A>>(
        self,
        other: G2,
    ) -> GetterImpl<S, A, ComposedGetter<Self, G2, S, I, A>> {
        GetterImpl::new(ComposedGetter::new(self, other))
    }

    pub fn compose_with_prism<A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
    ) -> PartialGetterImpl<
        S,
        A,
        ComposedPartialGetter<Self, PrismImpl<I, A, P2>, P2::GetterError, S, I, A>,
    > {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self, other, infallible, identity,
        ))
    }

    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> GetterImpl<S, A, ComposedGetter<Self, LensImpl<I, A, L2>, S, I, A>> {
        GetterImpl::new(ComposedGetter::new(self, other))
    }

    pub fn compose_with_fallible_iso<A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PartialGetterImpl<
        S,
        A,
        ComposedPartialGetter<Self, FallibleIsoImpl<I, A, F2>, F2::GetterError, S, I, A>,
    > {
        PartialGetterImpl::new(ComposedPartialGetter::new(
            self, other, infallible, identity,
        ))
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> GetterImpl<S, A, ComposedGetter<Self, IsoImpl<I, A, ISO2>, S, I, A>> {
        GetterImpl::new(ComposedGetter::new(self, other))
    }
}

pub fn identity_getter<S: Clone> () -> GetterImpl<S, S, impl Getter<S, S>> {
    mapped_getter(|x: &S| x.clone())
}