use crate::{
    FallibleIso, FallibleIsoImpl, Getter, GetterImpl, HasGetter, HasSetter, HasTotalGetter, Iso,
    IsoImpl, Lens, PartialGetter, PartialGetterImpl, Prism, PrismImpl, Setter, SetterImpl,
    composed_getter, composed_lens, composed_partial_getter, composed_prism, composed_setter,
    infallible,
};
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
/// This struct is not intended to be created by users directly, but it implements a From<`PartialGetter`<S,A>> so
/// that implementors of new optic types can wrap their concrete implementation of a `PartialGetter` optic.
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
    /// Composes this `LensImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If the second optic fails to focus, the composition will fail to focus.
    ///
    /// # Type Parameters
    ///
    /// - `A`: The target type of the composed optic.
    /// - `PG2`: The type of the partial getter to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The partial getter to compose with.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other`.
    ///
    pub fn compose_with_partial_getter<A, PG2: PartialGetter<I, A>>(
        self,
        other: PartialGetterImpl<I, A, PG2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = PG2::GetterError>> {
        composed_partial_getter(self.0, other.0, infallible, identity)
    }

    /// Composes this `LensImpl<S,I>` with a `GetterImpl<I,A>`, resulting in a new `GetterImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `GetterImpl` will extract a value by first applying `self` and then
    /// `other`.
    ///
    /// # Type Parameters
    ///
    /// - `A`: The target type of the composed optic.
    /// - `G2`: The type of the partial getter to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The getter to compose with.
    ///
    /// # Returns
    ///
    /// A new `GetterImpl` that represents the composition of `self` and `other`.
    ///
    pub fn compose_with_getter<A, G2: Getter<I, A>>(
        self,
        other: GetterImpl<I, A, G2>,
    ) -> GetterImpl<S, A, impl Getter<S, A>> {
        composed_getter(self.0, other.0)
    }

    /// Composes this `LensImpl<S,I>` with a `Setter<I,A>`, resulting in a new `Setter<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `SetterImpl` will attempt to set a value by first applying `self` and then
    /// `other`.
    ///
    /// # Type Parameters
    ///
    /// - `A`: The target type of the composed optic.
    /// - `S2`: The type of the setter to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The setter to compose with.
    ///
    /// # Returns
    ///
    /// A new `SetterImpl` that represents the composition of `self` and `other`.
    ///
    pub fn compose_with_setter<A, S2: Setter<I, A>>(
        self,
        other: SetterImpl<I, A, S2>,
    ) -> SetterImpl<S, A, impl Setter<S, A>> {
        composed_setter(self.0, other.0)
    }

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
