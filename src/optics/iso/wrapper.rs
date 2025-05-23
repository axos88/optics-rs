use crate::{
    FallibleIso, FallibleIsoImpl, Getter, GetterImpl, HasGetter, HasReverseGet, HasSetter,
    HasTotalGetter, HasTotalReverseGet, Iso, Lens, LensImpl, PartialGetter, PartialGetterImpl,
    Prism, PrismImpl, Setter, SetterImpl, composed_fallible_iso, composed_getter, composed_iso,
    composed_lens, composed_partial_getter, composed_prism, composed_setter, infallible,
};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

/// A wrapper of the [`Iso`] optic implementations, encapsulating a reversible bijective conversion.
///
/// `IsoImpl` provides a way to define a reversible bijective conversion - optics that can be used to
/// convert between values of type `A` and of type `S`.
/// This struct is particularly useful in scenarios where you need to deal with data types that have
/// multiple representations, such as a Point in cartesian and polar coordinates.
///
/// # Note
///
/// This struct is not intended to be created by users directly, but it implements a From<Iso<S,A>> so
/// that implementors of new optic types can wrap their concrete implementation of an Iso optic.
///
/// # Type Parameters
///
/// - `S`: The type the optic converts from
/// - `A`: The type the optic converts to
///
/// # See Also
///
/// - [`Iso`] trait for defining bijective conversions.
/// - [`mapped_iso`] function for creating `IsoImpl` instances from mapping functions.
pub struct IsoImpl<S, A, ISO: Iso<S, A>>(pub ISO, PhantomData<(S, A)>);

impl<S, A, ISO: Iso<S, A>> IsoImpl<S, A, ISO> {
    fn new(i: ISO) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        IsoImpl(i, PhantomData)
    }
}

impl<S, A, ISO: Iso<S, A>> From<ISO> for IsoImpl<S, A, ISO> {
    fn from(value: ISO) -> Self {
        Self::new(value)
    }
}

impl<S, A, ISO: Iso<S, A>> HasGetter<S, A> for IsoImpl<S, A, ISO> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, A, ISO: Iso<S, A>> HasSetter<S, A> for IsoImpl<S, A, ISO> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, ISO: Iso<S, A>> HasReverseGet<S, A> for IsoImpl<S, A, ISO> {
    type ReverseError = Infallible;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        Ok(self.0.reverse_get(value))
    }
}

impl<S, I, ISO1: Iso<S, I>> IsoImpl<S, I, ISO1> {
    /// Composes this `IsoImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
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

    /// Composes this `IsoImpl<S,I>` with a `GetterImpl<I,A>`, resulting in a new `GetterImpl<S, A>`
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

    /// Composes this `IsoImpl<S,I>` with a `Setter<I,A>`, resulting in a new `Setter<S, A>`
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
        composed_lens(self, other)
    }

    pub fn compose_with_prism<A, P2: Prism<I, A>>(
        self,
        other: P2,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = P2::GetterError>> {
        composed_prism(self, other, infallible, identity)
    }

    pub fn compose_with_fallible_iso<A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
    ) -> FallibleIsoImpl<
        S,
        A,
        impl FallibleIso<S, A, GetterError = FI2::GetterError, ReverseError = FI2::ReverseError>,
    > {
        composed_fallible_iso(self, other, infallible, identity, infallible, identity)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> IsoImpl<S, A, impl Iso<S, A>> {
        composed_iso(self.0, other.0)
    }
}
