use crate::{
    FallibleIso, Getter, GetterImpl, HasGetter, HasReverseGet, HasSetter, Iso, IsoImpl, Lens,
    LensImpl, PartialGetter, PartialGetterImpl, Prism, PrismImpl, Setter, SetterImpl,
    composed_fallible_iso, composed_partial_getter, composed_prism, composed_setter, infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

/// A wrapper of the [`FallibleIso`] optic implementations, encapsulating a potentially failing,
/// reversible bijective conversion.
///
/// `FallibleIsoImpl` provides a way to define a potentially failing reversible bijective conversion
/// optics that can be used to convert between values of type `A` and of type `S` where the
/// conversion may fail.
/// This struct is particularly useful in scenarios where you need to deal with data types that can
/// be converted to and from other types, but the conversion may not always succeed, such as an
/// `IpAddress` and a String.
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
/// - `GE`: The error type that can occur during the forward mapping.
/// - `RE`: The error type that can occur during the reverse mapping.
///
/// # See Also
///
/// - [`FallibleIso`] trait for defining bijective conversions.
/// - [`mapped_fallible_iso`] function for creating `FallibleIsoImpl` instances from mapping functions.
///
pub struct FallibleIsoImpl<S, A, FI: FallibleIso<S, A>>(pub FI, PhantomData<(S, A)>);

impl<S, A, FI: FallibleIso<S, A>> FallibleIsoImpl<S, A, FI> {
    pub fn new(l: FI) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        FallibleIsoImpl(l, PhantomData)
    }
}

impl<S, A, FI: FallibleIso<S, A>> HasGetter<S, A> for FallibleIsoImpl<S, A, FI> {
    type GetterError = FI::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, A, FI: FallibleIso<S, A>> HasSetter<S, A> for FallibleIsoImpl<S, A, FI> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, FI: FallibleIso<S, A>> HasReverseGet<S, A> for FallibleIsoImpl<S, A, FI> {
    type ReverseError = FI::ReverseError;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        self.0.try_reverse_get(value)
    }
}

impl<S, I, FI1: FallibleIso<S, I>> FallibleIsoImpl<S, I, FI1> {
    /// Composes this `FallibleIsoImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either optics fails to focus, the composition will fail to focus.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The error type for the composed partial getter, which must should be able to be constructed from
    ///   both `FI1::GetterError` and `PG2::GetterError` through `Into::into`.
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
    /// # Note
    ///
    /// This method uses `Into::into` to convert the errors from both prisms into the
    /// common error type `E`. If you need custom error mapping, consider using
    /// [`compose_with_partial_getter_with_mappers`](Self::compose_with_partial_getter_with_mappers).
    pub fn compose_with_partial_getter<E, A, PG2: PartialGetter<I, A>>(
        self,
        other: PartialGetterImpl<I, A, PG2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = E>>
    where
        FI1::GetterError: Into<E>,
        PG2::GetterError: Into<E>,
    {
        composed_partial_getter(self.0, other.0, Into::into, Into::into)
    }

    /// Composes this `FallibleIsoImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either optics fails to focus, the composition will fail to focus.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The common error type for the composed `partial_getter`.
    /// - `A`: The target type of the composed prism.
    ///
    /// # Parameters
    ///
    /// - `other`: The partial getter to compose with.
    /// - `error_mapper1`: A function to map `FI1::GetterError` into `E`.
    /// - `error_mapper2`: A function to map `PG2::GetterError` into `E`.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other` with
    /// custom error mapping.
    ///
    /// # Note
    ///
    /// This method is similar to [`compose_with_partial_getter`](Self::compose_with_partial_getter), but
    /// provides the ability to specify custom functions to map the errors from each
    /// optic into a common error type.
    pub fn compose_with_partial_getter_with_mappers<E, A, PG2: PartialGetter<I, A>>(
        self,
        other: PartialGetterImpl<I, A, PG2>,
        error_mapper_1: fn(FI1::GetterError) -> E,
        error_mapper_2: fn(PG2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = E>> {
        composed_partial_getter(self.0, other.0, error_mapper_1, error_mapper_2)
    }

    /// Composes this `FallibleIsoImpl<S,I>` with a `Getter<I,A>`, resulting in a new `PartialGetter<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If the first optic fails to focus, the composition will fail to focus.
    ///
    /// # Type Parameters
    ///
    /// - `A`: The target type of the composed optic.
    /// - `G2`: The type of the getter to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The getter to compose with.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other`.
    ///
    pub fn compose_with_getter<A, G2: Getter<I, A>>(
        self,
        other: GetterImpl<I, A, G2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = FI1::GetterError>> {
        composed_partial_getter(self.0, other.0, identity, infallible)
    }

    /// Composes this `FallibleIsoImpl<S,I>` with a `Setter<I,A>`, resulting in a new `Setter<S, A>`
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

    pub fn compose_with_prism<E, A, P2: Prism<I, A>>(
        self,
        other: P2,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>>
    where
        E: From<FI1::GetterError> + From<P2::GetterError>,
    {
        composed_prism(self, other, Into::into, Into::into)
    }

    pub fn compose_with_prism_with_mappers<E, A, P2: Prism<I, A>>(
        self,
        other: P2,
        error_mapper_1: fn(FI1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>> {
        composed_prism(self, other, error_mapper_1, error_mapper_2)
    }

    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = FI1::GetterError>> {
        composed_prism(self, other, identity, infallible)
    }

    pub fn compose_with_fallible_iso<GE, RE, A, FI2: FallibleIso<I, A>>(
        self,
        other: FI2,
    ) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A, GetterError = GE, ReverseError = RE>>
    where
        GE: From<FI1::GetterError> + From<FI2::GetterError>,
        RE: From<FI1::ReverseError> + From<FI2::ReverseError>,
    {
        composed_fallible_iso(self, other, Into::into, Into::into, Into::into, Into::into)
    }

    pub fn compose_with_fallible_iso_with_mappers<GE, RE, A, FI2: FallibleIso<I, A>>(
        self,
        other: FI2,
        getter_error_mapper_1: fn(FI1::GetterError) -> GE,
        getter_error_mapper_2: fn(FI2::GetterError) -> GE,
        reverse_error_mapper_1: fn(FI1::ReverseError) -> RE,
        reverse_error_mapper_2: fn(FI2::ReverseError) -> RE,
    ) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A, GetterError = GE, ReverseError = RE>> {
        composed_fallible_iso(
            self,
            other,
            getter_error_mapper_1,
            getter_error_mapper_2,
            reverse_error_mapper_1,
            reverse_error_mapper_2,
        )
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A>> {
        composed_fallible_iso(self.0, other.0, identity, infallible, identity, infallible)
    }
}
