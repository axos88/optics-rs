use crate::optics::prism::composed::new as composed_prism;
use crate::{
    FallibleIso, FallibleIsoImpl, Getter, GetterImpl, HasGetter, HasSetter, Iso, IsoImpl, Lens,
    LensImpl, PartialGetter, PartialGetterImpl, Prism, Setter, SetterImpl, composed_partial_getter,
    composed_setter, infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

/// A wrapper of the [`Prism`] optic implementations, encapsulating a partial getter and a setter function.
///
/// `Prism` provides a way to define optics that can focus on a potentially missing value of type `A`
/// from a source of type `S`, such as the variant of an enum.
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
/// - [`Prism`] an optic that focuses on a potentially missing value.
/// - [`mapped_prism`] function for creating `PrismImpl` instances from mapping functions.
pub struct PrismImpl<S, A, P: Prism<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, P: Prism<S, A>> PrismImpl<S, A, P> {
    fn new(prism: P) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        PrismImpl(prism, PhantomData)
    }
}

impl<S, A, P: Prism<S, A>> From<P> for PrismImpl<S, A, P> {
    fn from(value: P) -> Self {
        Self::new(value)
    }
}

impl<S, A, P: Prism<S, A>> HasGetter<S, A> for PrismImpl<S, A, P> {
    type GetterError = P::GetterError;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        self.0.try_get(source)
    }
}

impl<S, A, P: Prism<S, A>> HasSetter<S, A> for PrismImpl<S, A, P> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, I, P1: Prism<S, I>> PrismImpl<S, I, P1> {
    /// Composes this `PrismImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
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
        P1::GetterError: Into<E>,
        PG2::GetterError: Into<E>,
    {
        composed_partial_getter(self.0, other.0, Into::into, Into::into)
    }

    /// Composes this `PrismImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
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
    /// - `error_mapper1`: A function to map `P1::GetterError` into `E`.
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
        error_mapper_1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(PG2::GetterError) -> E,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = E>> {
        composed_partial_getter(self.0, other.0, error_mapper_1, error_mapper_2)
    }

    /// Composes this `PrismImpl<S,I>` with a `Getter<I,A>`, resulting in a new `PartialGetter<S, A>`
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
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = P1::GetterError>> {
        composed_partial_getter(self.0, other.0, identity, infallible)
    }

    /// Composes this `PrismImpl<S,I>` with a `Setter<I,A>`, resulting in a new `Setter<S, A>`
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
    /// A new `PartialGetterImpl` that represents the composition of `self` and `other`.
    ///
    pub fn compose_with_setter<A, S2: Setter<I, A>>(
        self,
        other: SetterImpl<I, A, S2>,
    ) -> SetterImpl<S, A, impl Setter<S, A>> {
        composed_setter(self.0, other.0)
    }

    /// Composes this `PrismImpl<S,I>` with another `Prism<I,A>`, resulting in a new `PrismImpl<S, A>`
    /// that focuses through both prisms sequentially.
    ///
    /// The resulting `PrismImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either prism fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The error type for the composed prism, which must should be able to be constructed from
    ///   both `P1::GetterError` and `P2::GetterError` through `Into::into`.
    /// - `A`: The target type of the composed prism.
    /// - `P2`: The type of the second prism to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The second prism to compose with.
    ///
    /// # Returns
    ///
    /// A new `PrismImpl` that represents the composition of `self` and `other`.
    ///
    /// # Note
    ///
    /// This method uses `Into::into` to convert the errors from both prisms into the
    /// common error type `E`. If you need custom error mapping, consider using
    /// [`compose_with_prism_with_mappers`](Self::compose_with_prism_with_mappers).
    pub fn compose_with_prism<E, A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>>
    where
        P1::GetterError: Into<E>,
        P2::GetterError: Into<E>,
    {
        composed_prism(self.0, other.0, Into::into, Into::into)
    }

    /// Composes this `PrismImpl<S,I>` with another `PrismImpl<I,A>`, resulting in a new `PrismImpl<S, A>`
    /// that focuses through both prisms sequentially.
    ///
    /// The resulting `PrismImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either prism fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The common error type for the composed prism.
    /// - `A`: The target type of the composed prism.
    ///
    /// # Parameters
    ///
    /// - `other`: The second prism to compose with.
    /// - `error_mapper1`: A function to map `P1::GetterError` into `E`.
    /// - `error_mapper2`: A function to map `P2::GetterError` into `E`.
    ///
    /// # Returns
    ///
    /// A new `PrismImpl` that represents the composition of `self` and `other` with
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
        error_mapper_1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>> {
        composed_prism(self.0, other.0, error_mapper_1, error_mapper_2)
    }

    /// Composes this `PrismImpl<S,I>` with a `Lens<I,A>`, resulting in a new `PrismImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PrismImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If the prism fails to match, the composition will fail.
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
    /// A new `PrismImpl` that represents the composition of `self` and `other`
    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = P1::GetterError>> {
        composed_prism(self.0, other.0, identity, infallible)
    }

    /// Composes this `PrismImpl<S,I>` with a `FallibleIso<I,A>`, resulting in a new `PrismImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PrismImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either prism fails to match, the composition will fail.
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
    /// A new `PrismImpl` that represents the composition of `self` and `other`.
    ///
    /// # Note
    ///
    /// This method uses `Into::into` to convert the errors from both prisms into the
    /// common error type `E`. If you need custom error mapping, consider using
    /// [`compose_with_fallible_iso_with_mappers`](Self::compose_with_fallible_iso_with_mappers).
    pub fn compose_with_fallible_iso<E, A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>>
    where
        FI2::GetterError: Into<E>,
        P1::GetterError: Into<E>,
    {
        composed_prism(self.0, other.0, Into::into, Into::into)
    }

    /// Composes this `PrismImpl<S,I>` with a `FallibleIso<I,A>`, resulting in a new `PrismImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PrismImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either prism fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The common error type for the composed prism.
    /// - `A`: The target type of the composed prism.
    /// - `F2`: The type of the fallible iso to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The fallible iso to compose with.
    /// - `error_mapper1`: A function to map `P1::GetterError` into `E`.
    /// - `error_mapper2`: A function to map `F2::GetterError` into `E`.
    ///
    /// # Returns
    ///
    /// A new `PrismImpl` that represents the composition of `self` and `other` with
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
        getter_error_mapper_1: fn(P1::GetterError) -> E,
        getter_error_mapper_2: fn(FI2::GetterError) -> E,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>> {
        composed_prism(
            self.0,
            other.0,
            getter_error_mapper_1,
            getter_error_mapper_2,
        )
    }

    /// Composes this `PrismImpl<S,I>` with an `Iso<I,A>`, resulting in a new `PrismImpl<S, A>`
    /// that focuses through both optics sequentially.
    ///
    /// The resulting `PrismImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If the prism fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `A`: The target type of the composed prism.
    /// - `ISO2`: The type of the lens to compose with.
    ///
    /// # Parameters
    ///
    /// - `other`: The iso to compose with.
    ///
    /// # Returns
    ///
    /// A new `PrismImpl` that represents the composition of `self` and `other`
    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = P1::GetterError>> {
        composed_prism(self.0, other.0, identity, infallible)
    }
}
