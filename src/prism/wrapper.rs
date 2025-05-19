use crate::prism::composed::new as composed_prism;
use crate::{
    FallibleIso, FallibleIsoImpl, HasGetter, HasSetter, Iso, IsoImpl, Lens, LensImpl, Prism,
    infallible,
};
use core::convert::identity;
use core::marker::PhantomData;

/// Concrete implementation wrapper for a `Prism` optic.
///
/// This struct provides a public, coherent-safe wrapper around any type implementing
/// the [`Prism`] trait, and serves as the primary public-facing API for working with prisms
/// in downstream code.
///
/// # Design
///
/// Due to Rust's coherence rules, we can't blanket-implement traits like `Prism`
/// for downstream-defined types that might conflict with other implementations.
/// To work around this, optics are wrapped in an `Impl` newtype struct like `PrismImpl`,
/// which implements the optic's base traits (`HasPartialGetter`, `HasSetter`) as well as
/// transitive optics (`PartialGetter`, `Setter`, `Prism` itself).
///
/// This ensures that all public APIs return `PrismImpl`, while the internal implementations
/// remain opaque and private to the crate.
///
/// # Composition
///
/// `PrismImpl` provides several `compose_with_*` methods to compose this prism with other
/// optic types. Each of these returns a new `*Impl` wrapping a `Composed*`
/// containing the two optics and any necessary error mappers.
///
/// The resulting optic type is determined by the composition rules defined in the
/// crate's optic type system (see the composition table in the crate root documentation).
///
/// # Default Prism
///
/// The [`crate::identity_prism`] function returns a trivial identity prism, which always succeeds
/// in previewing by cloning the source, and performs a no-op on `set`.
///
/// # See Also
///
/// - [`Prism`] — The marker trait defining the core behavior.
/// - [`ComposedPrism`] — The struct representing the composition of two optics resulting in a prism.
/// - [`MappedPrism`] — A prism implementation using custom getter and setter functions.
pub struct PrismImpl<S, A, P: Prism<S, A>>(pub P, PhantomData<(S, A)>);

impl<S, A, P: Prism<S, A>> PrismImpl<S, A, P> {
    /// Constructs a new [`PrismImpl`] wrapper around a [`Prism`] implementation.
    ///
    /// This constructor is the primary entry point for creating a `PrismImpl`, which serves as the public-facing
    /// interface for prism optics within this crate. By encapsulating the provided `Prism` implementation, it
    /// ensures that it can be cast to lower optic types like `PartialGetter` and `Setter`.
    ///
    /// # Parameters
    ///
    /// - `prism`: A concrete implementation of the [`Prism`] trait, defining the core behavior of the optic.
    ///
    /// # Returns
    ///
    /// A new instance of [`PrismImpl`] that wraps the provided `Prism` implementation, ready for use in optic
    /// compositions and transformations.
    ///
    /// # Notes
    ///
    /// Generally only used directly when prism implementations are created outside the crate
    fn new(prism: P) -> Self {
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

/// Composition methods for chaining a `PrismImpl` with other optic types,
/// resulting in a new composed optic.
///
/// These methods return a new [`*Impl`] wrapping a [`Composed*`].
///
/// Error mappers can be provided when composing partial optics to reconcile their error types.
impl<S, I, P1: Prism<S, I>> PrismImpl<S, I, P1> {
    //TODO: Partial Getter, Getter, Setter

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
        other: P2,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>>
    where
        P1::GetterError: Into<E>,
        P2::GetterError: Into<E>,
    {
        composed_prism(self, other, Into::into, Into::into)
    }

    /// Composes this `PrismImpl<S,I>` with another `Prism<I,A>`, resulting in a new `PrismImpl<S, A>`
    /// that focuses through both prisms sequentially.
    ///
    /// The resulting `PrismImpl` will attempt to extract a value by first applying `self` and then
    /// `other`. If either prism fails to match, the composition will fail.
    ///
    /// # Type Parameters
    ///
    /// - `E`: The common error type for the composed prism.
    /// - `A`: The target type of the composed prism.
    /// - `P2`: The type of the second prism to compose with.
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
        other: P2,
        error_mapper1: fn(P1::GetterError) -> E,
        error_mapper_2: fn(P2::GetterError) -> E,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>> {
        composed_prism(self, other, error_mapper1, error_mapper_2)
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
        composed_prism(self, other, identity, infallible)
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
    pub fn compose_with_fallible_iso<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>>
    where
        F2::GetterError: Into<E>,
        P1::GetterError: Into<E>,
    {
        composed_prism(self, other, Into::into, Into::into)
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
    pub fn compose_with_fallible_iso_with_mappers<E, A, F2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, F2>,
        getter_error_mapper_1: fn(P1::GetterError) -> E,
        getter_error_mapper_2: fn(F2::GetterError) -> E,
    ) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>> {
        composed_prism(self, other, getter_error_mapper_1, getter_error_mapper_2)
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
    /// - `L2`: The type of the lens to compose with.
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
        composed_prism(self, other, identity, infallible)
    }
}
