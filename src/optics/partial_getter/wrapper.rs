use crate::optics::partial_getter::composed::new as composed_partial_getter;
use crate::{
    FallibleIso, FallibleIsoImpl, Getter, GetterImpl, HasGetter, Iso, IsoImpl, Lens, LensImpl,
    PartialGetter, Prism, PrismImpl, Setter, SetterImpl, infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

/// A wrapper of the [`PartialGetter`] optic implementations, encapsulating a partial getter function.
///
/// `PartialGetterImpl` provides a way to define partial getters - optics that attempt to retrieve
/// a value of type `A` from a source of type `S`, potentially failing with an error.
/// This struct is particularly useful in scenarios where you need to compose or reuse getter logic
/// that might not always succeed.
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
/// - [`PartialGetter`] trait for defining custom partial getters.
/// - [`mapped_partial_getter`] function for creating `PartialGetterImpl` instances from mapping functions.
pub struct PartialGetterImpl<S, A, PG: PartialGetter<S, A>>(pub PG, PhantomData<(S, A)>);

impl<S, A, PG: PartialGetter<S, A>> PartialGetterImpl<S, A, PG> {
    fn new(prism: PG) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        PartialGetterImpl(prism, PhantomData)
    }
}

impl<S, A, PG: PartialGetter<S, A>> From<PG> for PartialGetterImpl<S, A, PG> {
    fn from(value: PG) -> Self {
        Self::new(value)
    }
}

impl<S, A, PG: PartialGetter<S, A>> HasGetter<S, A> for PartialGetterImpl<S, A, PG> {
    type GetterError = PG::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, I, PG1: PartialGetter<S, I>> PartialGetterImpl<S, I, PG1> {
    /// Composes this `PartialGetterImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either optics fails to focus, the composition will fail to focus.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The error type for the composed partial getter, which must should be able to be constructed from
    ///   both `P1::GetterError` and `PG2::GetterError` through `Into::into`.
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
        PG1::GetterError: Into<E>,
        PG2::GetterError: Into<E>,
    {
        composed_partial_getter(self.0, other.0, Into::into, Into::into)
    }

    /// Composes this `PartialGetterImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
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
    /// - `error_mapper1`: A function to map `PG1::GetterError` into `E`.
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
        error_mapper_1: fn(PG1::GetterError) -> E,
        error_mapper_2: fn(PG2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self.0, other.0, error_mapper_1, error_mapper_2)
    }

    /// Composes this `PartialGetterImpl<S,I>` with a `GetterImpl<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
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
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self.0, other.0, identity, infallible)
    }

    /// Impossible to combine
    /// # Panics
    /// always
    pub fn compose_with_setter<A, S2: Setter<I, A>>(self, _other: SetterImpl<I, A, S2>) -> !
    where
        PG1: Prism<S, I>,
    {
        panic!()
    }

    /// Composes this `PartialGetterImpl<S,I>` with another `Prism<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
    /// that focuses through both prisms sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either optics fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The error type for the composed partial getter, which must should be able to be constructed from
    ///   both `P1::GetterError` and `P2::GetterError` through `Into::into`.
    /// - `A`: The target type of the composed prism.
    /// - `P2`: The type of the prism to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The prism to compose with.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other`.
    ///
    /// # Note
    ///
    /// This method uses `Into::into` to convert the errors from both prisms into the
    /// common error type `E`. If you need custom error mapping, consider using
    /// [`compose_with_prism_with_mappers`](Self::compose_with_prism_with_mappers).
    pub fn compose_with_prism<E, A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        PG1::GetterError: Into<E>,
        P2::GetterError: Into<E>,
    {
        composed_partial_getter(self.0, other.0, Into::into, Into::into)
    }

    /// Composes this `PartialGetterImpl<S,I>` with a `PrismImpl<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
    /// that focuses through both prisms sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either optics fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The common error type for the composed prism.
    /// - `A`: The target type of the composed prism.
    ///
    /// # Parameters
    ///
    /// - `other`: The second prism to compose with.
    /// - `error_mapper1`: A function to map `PG1::GetterError` into `E`.
    /// - `error_mapper2`: A function to map `P2::GetterError` into `E`.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other` with
    /// custom error mapping.
    ///
    /// # Note
    ///
    /// This method is similar to [`compose_with_prism`](Self::compose_with_prism), but
    /// provides the ability to specify custom functions to map the errors from each
    /// prism into a common error type.
    pub fn compose_with_prism_with_mappers<E, A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
        error_mapper_1: fn(PG1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self.0, other.0, error_mapper_1, error_mapper_2)
    }

    /// Composes this `PartialGetterImpl<S,I>` with a `Lens<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If the partial getter fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `A`: The target type of the composed prism.
    /// - `L2`: The type of the lens to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The lens to compose with.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other`
    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(self.0, other.0, identity, infallible)
    }

    /// Composes this `PartialGetterImpl<S,I>` with a `FallibleIso<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either optics fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The error type for the composed prism, which must should be able to be constructed from
    ///   both `P1::GetterError` and `P2::GetterError` through `Into::into`.
    /// - `A`: The target type of the composed prism.
    /// - `F2`: The type of the fallible iso to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The second Fallible Iso to compose with.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other`.
    ///
    /// # Note
    ///
    /// This method uses `Into::into` to convert the errors from both prisms into the
    /// common error type `E`. If you need custom error mapping, consider using
    /// [`compose_with_fallible_iso_with_mappers`](Self::compose_with_fallible_iso_with_mappers).
    pub fn compose_with_fallible_iso<E, A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>>
    where
        E: From<FI2::GetterError> + From<PG1::GetterError>,
    {
        composed_partial_getter(self.0, other.0, Into::<E>::into, Into::<E>::into)
    }

    /// Composes this `PartialGetterImpl<S,I>` with a `FallibleIso<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either prism fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The common error type for the composed prism.
    /// - `A`: The target type of the composed prism.
    /// - `FI2`: The type of the fallible iso to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The fallible iso to compose with.
    /// - `error_mapper1`: A function to map `P1::GetterError` into `E`.
    /// - `error_mapper2`: A function to map `F2::GetterError` into `E`.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other` with
    /// custom error mapping.
    ///
    /// # Note
    ///
    /// This method is similar to [`compose_with_fallible_iso`](Self::compose_with_fallible_iso), but
    /// provides the ability to specify custom functions to map the errors from each
    /// prism into a common error type.
    pub fn compose_with_fallible_iso_with_mappers<E, A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
        getter_error_mapper_1: fn(PG1::GetterError) -> E,
        getter_error_mapper_2: fn(FI2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A>> {
        composed_partial_getter(
            self.0,
            other.0,
            getter_error_mapper_1,
            getter_error_mapper_2,
        )
    }

    /// Composes this `PartialGetterImpl<S,I>` with an `Iso<I,A>`, resulting in a new `PartialGetterImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PartialGetterImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If the prism fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `A`: The target type of the composed prism.
    /// - `PG2`: The type of the lens to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The iso to compose with.
    ///
    /// # Returns
    ///
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other`
    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = PG1::GetterError>> {
        composed_partial_getter(self.0, other.0, identity, infallible)
    }
}
