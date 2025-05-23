use crate::optics::getter::composed::new as composed_getter;
use crate::{
    FallibleIso, FallibleIsoImpl, Getter, HasGetter, HasTotalGetter, Iso, IsoImpl, Lens, LensImpl,
    PartialGetter, PartialGetterImpl, Prism, PrismImpl, Setter, SetterImpl,
    composed_partial_getter, composed_setter, infallible,
};
use core::convert::{Infallible, identity};
use core::marker::PhantomData;

/// A wrapper of the [`Getter`] optic implementations, encapsulating a total getter function.
///
/// `GetterImpl` provides a way to define total getters - optics that retrieve
/// a value of type `A` from a source of type `S`.
/// This struct is particularly useful in scenarios where you need to compose or reuse getter logic
/// that always succeeds.
///
/// # Note
///
/// This struct is not intended to be created by users directly, but it implements a From<Getter<S,A>> so
/// that implementors of new optic types can wrap their concrete implementation of a Getter optic.
///
/// # Type Parameters
///
/// - `S`: The source type from which the value is to be retrieved.
/// - `A`: The target type of the value to be retrieved.
///
/// # See Also
///
/// - [`Getter`] trait for defining custom partial getters.
/// - [`mapped_getter`] function for creating `GetterImpl` instances from mapping functions.
pub struct GetterImpl<S, A, G: Getter<S, A>>(pub G, PhantomData<(S, A)>);

impl<S, A, G: Getter<S, A>> From<G> for GetterImpl<S, A, G> {
    fn from(value: G) -> Self {
        Self::new(value)
    }
}

impl<S, A, G: Getter<S, A>> GetterImpl<S, A, G> {
    fn new(prism: G) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        GetterImpl(prism, PhantomData)
    }
}

impl<S, A, G: Getter<S, A>> HasGetter<S, A> for GetterImpl<S, A, G> {
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.0.get(source))
    }
}

impl<S, I, G1: Getter<S, I>> GetterImpl<S, I, G1> {
    /// Composes this `GetterImpl<S,I>` with a `PartialGetter<I,A>`, resulting in a new `PartialGetter<S, A>`
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

    /// Composes this `GetterImpl<S,I>` with a `GetterImpl<I,A>`, resulting in a new `GetterImpl<S, A>`
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

    /// Composes this `GetterImpl<S,I>` with a `Setter<I,A>`, resulting in a new `Setter<S, A>`
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
    ) -> SetterImpl<S, A, impl Setter<S, A>>
    where
        G1: Setter<S, I>,
    {
        composed_setter(self.0, other.0)
    }

    pub fn compose_with_prism<A, P2: Prism<I, A>>(
        self,
        other: PrismImpl<I, A, P2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = P2::GetterError>> {
        composed_partial_getter(self, other, infallible, identity)
    }

    pub fn compose_with_lens<A, L2: Lens<I, A>>(
        self,
        other: LensImpl<I, A, L2>,
    ) -> GetterImpl<S, A, impl Getter<S, A>> {
        composed_getter(self, other.0)
    }

    pub fn compose_with_fallible_iso<A, FI2: FallibleIso<I, A>>(
        self,
        other: FallibleIsoImpl<I, A, FI2>,
    ) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = FI2::GetterError>> {
        composed_partial_getter(self, other.0, infallible, identity)
    }

    pub fn compose_with_iso<A, ISO2: Iso<I, A>>(
        self,
        other: IsoImpl<I, A, ISO2>,
    ) -> GetterImpl<S, A, impl Getter<S, A>> {
        composed_getter(self, other.0)
    }
}
