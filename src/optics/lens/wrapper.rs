use crate::{FallibleIso, FallibleIsoImpl, HasGetter, HasSetter, HasTotalGetter, Iso, IsoImpl, Lens, Prism, PrismImpl, composed_lens, composed_prism, infallible, PartialGetter};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

/// A wrapper of the [`Lens`] optic implementations, encapsulating a getter and setter function.
///
/// `LensImpl` provides a way to define lenses - optics that can retrieve and change a value of 
/// type `A` inside a source of type `S`,
/// They are particularly useful in scenarios where you need to focus on a field of a struct.
///
/// # Note
///
/// This struct is not intended to be created by users directly, but it implements a From<PartialGetter<S,A>> so
/// that implementors of new optic types can wrap their concrete implementation of a PartialGetter optic.
///
/// # Type Parameters
///
/// - `S`: The source type from which the value is to be retrieved.
/// - `A`: The target type of the value to be retrieved.
///
/// # See Also
///
/// - [`Lens`] trait for defining custom partial getters.
/// - [`mapped_lens`] function for creating `LebsImpl` instances from mapping functions.
pub struct LensImpl<S, A, L: Lens<S, A>>(pub L, PhantomData<(S, A)>);

impl<S, A, L: Lens<S, A>> LensImpl<S, A, L> {
    fn new(l: L) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        LensImpl(l, PhantomData)
    }
}

impl<S, A, L: Lens<S, A>> From<L> for LensImpl<S, A, L> {
    fn from(value: L) -> Self {
        Self::new(value)
    }
}

impl<S, A, L: Lens<S, A>> HasGetter<S, A> for LensImpl<S, A, L> {
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

impl<S, I, L: Lens<S, I>> LensImpl<S, I, L> {
    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> LensImpl<S, A, impl Lens<S, A>> {
        composed_lens(self.0, other.0)
    }

    pub fn compose_with_prism<A, P: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = P::GetterError>> {
        composed_prism(self.0, other.0, infallible, identity)
    }

    pub fn compose_with_fallible_iso<A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = FI2::GetterError>> {
        composed_prism(self, other, infallible, identity)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> LensImpl<S, A, impl Lens<S, A>> {
        composed_lens(self.0, other.0)
    }
}
