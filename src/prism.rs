use crate::fallible_iso::FallibleIso;
use crate::iso::Iso;
use crate::lens::Lens;
use crate::optic::Optic;
use core::convert::Infallible;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// An error type indicating that a `Prism` failed to extract a focus value.
///
/// A `Prism` represents an optional focus within a sum type, and attempting to
/// access a value using its `preview` or similar method may fail if the underlying
/// value is in a different variant or does not contain the expected focus.
///
/// `NoFocus` serves as a simple, lightweight error type to signal this situation.
pub struct NoFocus;

impl From<Infallible> for NoFocus {
    fn from(_value: Infallible) -> Self {
        NoFocus
    }
}

/// A `Prism` is an optic that focuses on a potential value inside a sum type.
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
/// `Prism` is currently the most general form of a *concrete optic* in this library.
/// In the future, it may be generalized further to allow any error type in place
/// of the fixed `NoFocus` error. When that happens, `Prism<S, A>` will become a
/// special case of a fully general `Optic<S, A>` abstraction, making this trait
/// redundant and subject to removal in favor of the unified `Optic<S, A>` design.
///
/// This would allow error type specialization while preserving the same core behavior.
///
/// # See Also
/// - [`Optic`] — the base trait for all optic types, potentially unifying `Lens`, `Prism`, and `Iso`
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the forward mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible one-to-one transformation between two types
///
/// - [`NoFocus`] — the current error type returned by `Prism::preview` on failure
pub trait Prism<S, A>: Optic<S, A>
where
    Self::Error: Into<NoFocus>,
{
    /// Attempt to extract a value of type `A` from `S`
    fn preview(&self, source: &S) -> Option<A>;
}

/// A concrete implementation of the [`Prism`] trait.
///
/// This struct allows you to create a `Prism` by providing custom getter and setter functions.
/// It is the primary way to create a `Prism` manually, and is flexible enough to support any
/// getter and setter functions that match the required signatures.
///
/// Typically, you will only need to specify the source type `S` and the focus type `A`.
/// The getter and setter functions will be inferred or explicitly provided.
///
/// If you prefer to use capturing closures, you can specify the trailing type parameters as `_`
/// in the construction of `PrismImpl` to allow the compiler to infer them for you. This is especially
/// useful when you need to use closures with captured environment variables.
///
/// # Construction
///
/// The usual way to construct a `PrismImpl` is to use `PrismImpl::<S, A>::new()`, which
/// will use non-capturing closures by default. Alternatively, you can specify the
/// getter and setter type parameters as `PrismImpl::<S, A,_, _>::new()` for more complex use cases,
/// such as captruring closures.
///
/// # See Also
///
/// - [`Lens`] — a more restrictive optic type for focus values
/// - [`Optic`] — base trait that all optics implement
pub struct PrismImpl<S, A, GET = fn(&S) -> Option<A>, SET = fn(&mut S, A)>
where
    GET: Fn(&S) -> Option<A>,
    SET: Fn(&mut S, A),
{
    get_fn: GET,
    set_fn: SET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, GET, SET> PrismImpl<S, A, GET, SET>
where
    GET: Fn(&S) -> Option<A>,
    SET: Fn(&mut S, A),
{
    /// Creates a new `LensImpl` with the provided getter and setter functions.
    ///
    /// # Arguments
    ///
    /// - `get_fn` — A function that retrieves the focus value `A` from the source `S`.
    /// - `set_fn` — A function that sets the focus value `A` in the source `S`.
    ///
    /// # Returns
    ///
    /// A new `LensImpl` instance that can be used as a `Lens<S, A>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use optics::{Lens, LensImpl, Optic};
    ///
    /// struct Point { x: i32, y: i32 }
    /// let mut point = Point { x: 10, y: 20 };
    /// let x_lens = LensImpl::<Point, i32>::new(
    ///     |p| p.x,
    ///     |p, new_x| p.x = new_x
    /// );
    /// let x_value = x_lens.get(&point); // retrieves 10
    /// x_lens.set(&mut point, 30); // sets x to 30
    /// ```
    ///
    /// # Capturing Closures
    ///
    /// You can also use capturing closures for more flexible behavior, such as when you
    /// need to capture environment variables. In that case, you can specify the trailing
    /// type parameters as `_`, and the compiler will infer them:
    ///
    /// ```
    /// use optics::{Lens, LensImpl, Optic};
    ///
    /// struct Point { x: i32, y: i32 }
    /// let factor = 2;
    /// let mut point = Point { x: 10, y: 20 };
    /// let x_lens = LensImpl::<Point, i32, _, _>::new(
    ///     move |p| p.x * factor,
    ///     move |p, new_x| p.x = new_x / factor
    /// );
    /// let x_value = x_lens.get(&point); // retrieves 10 * 2 = 20
    /// x_lens.set(&mut point, 60); // sets x to 60 / 2 = 30
    /// ```
    pub fn new(get_fn: GET, set_fn: SET) -> Self {
        PrismImpl {
            get_fn,
            set_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, GET, SET> Optic<S, A> for PrismImpl<S, A, GET, SET>
where
    GET: Fn(&S) -> Option<A>,
    SET: Fn(&mut S, A),
{
    type Error = NoFocus;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        (self.get_fn)(source).ok_or(NoFocus)
    }

    fn set(&self, source: &mut S, value: A) {
        (self.set_fn)(source, value);
    }
}

impl<S, A, GET, SET> Prism<S, A> for PrismImpl<S, A, GET, SET>
where
    GET: Fn(&S) -> Option<A>,
    SET: Fn(&mut S, A),
{
    fn preview(&self, source: &S) -> Option<A> {
        (self.get_fn)(source)
    }
}

/// A trait for composing a `Prism` with other optic types.
///
/// This trait enables the composition of a `Prism` with other types of optics, such as `Lens`,
/// `Iso`, `FallibleIso`, and another `Prism`.
///
/// # Type Parameters
/// - `S`: The source type that the `Prism` operates on.
/// - `I`: The intermediate type in the optic composition.
/// - `A`: The final target type that the optic composition focuses on.
/// - `O2`: The type of the other optic being composed with this `Prism`.
///
/// # Methods
/// The methods in this trait allow composing a `Prism` with various optic types, returning a composed
/// optic that represents a combined focus on the data. Each method corresponds to a different way of
/// composing optics, resulting in a new type of optic
///
/// # See Also
/// - [`Prism`] — the core trait that defines prism-based optics.
/// - [`Lens`] — a trait for working with lens-based optics, a subset of `Optic`.
/// - [`Iso`] — a trait for reversible transformations between types.
/// - [`FallibleIso`] — a trait for isomorphisms that may fail.
/// - [`ComposedPrism`] — a type representing the possible result of composing a lens with other optics.
pub trait ComposablePrism<S, I, A, O2: Optic<I, A>>: Prism<S, I> + Sized
where
    Self::Error: Into<NoFocus>,
{
    /// Composes the current `Prism` with a `Lens`.
    ///
    /// This method combines a `Prism` with a `Lens`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Lens` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    fn compose_prism_with_lens(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: Lens<I, A> + Prism<I, A>;

    /// Composes the current `Prism` with a `Prism`.
    ///
    /// This method combines a `Prism` with a `Prism`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Prism` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    fn compose_prism_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: Prism<I, A>,
        O2::Error: Into<NoFocus>;

    /// Composes the current `Prism` with a `FallibleIso`.
    ///
    /// This method combines a `Prism` with a `FallibleIso`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `FallibleIso` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    fn compose_prism_with_fallible_iso<E>(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: FallibleIso<I, A> + Prism<I, A>,
        E: From<Self::Error> + From<O2::Error>,
        O2::Error: Into<NoFocus>;

    /// Composes the current `Prism` with an `Iso`.
    ///
    /// This method combines a `Prism` with an `Iso`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Prism`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    fn compose_prism_with_iso(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: Iso<I, A> + Prism<I, A>;
}

impl<P, O2, S, I, A> ComposablePrism<S, I, A, O2> for P
where
    P: Prism<S, I> + Sized,
    Self::Error: Into<NoFocus>,
    O2: Optic<I, A>,
{
    fn compose_prism_with_lens(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: Lens<I, A> + Prism<I, A>,
        O2::Error: Into<NoFocus>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_prism_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: Prism<I, A>,
        O2::Error: Into<NoFocus>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_prism_with_fallible_iso<E>(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: FallibleIso<I, A> + Prism<I, A>,
        E: From<Self::Error> + From<O2::Error>,
        O2::Error: Into<NoFocus>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_prism_with_iso(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        O2: Iso<I, A> + Prism<I, A>,
        O2::Error: Into<NoFocus>,
    {
        ComposedPrism::new(self, other)
    }
}

/// A composed `Prism` type, combining two optics into a single prism.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type that results in a `Prism`.
///
/// A `ComposedPrism` not only combines two optics into a single [`Prism`], but it also inherently
/// acts as an `Optic`. This behavior arises from the fact that a `Prism` is itself a
/// more specific form of an optic, and thus any `Prism` composition will also be usable an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedPrism` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Prism`] — the optic type that `ComposedPrism` is based on
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableLens`] — a trait for composing [`Lens`] optics another [`Optic`]
/// - [`crate::composers::ComposablePrism`] — a trait for composing [`Prism`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
pub struct ComposedPrism<O1, O2, S, I, A> {
    optic1: O1,
    optic2: O2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<O1, O2, S, I, A> ComposedPrism<O1, O2, S, I, A> {
    pub(crate) fn new(optic1: O1, optic2: O2) -> Self
    where
        O1: Prism<S, I>,
        O2: Prism<I, A>,
        O1::Error: Into<NoFocus>,
        O2::Error: Into<NoFocus>,
    {
        ComposedPrism {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<O1, O2, S, I, A> Optic<S, A> for ComposedPrism<O1, O2, S, I, A>
where
    O1: Optic<S, I>,
    O2: Optic<I, A>,
    O1::Error: Into<NoFocus>,
    O2::Error: Into<NoFocus>,
{
    type Error = NoFocus;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.optic2
            .try_get(&self.optic1.try_get(source).map_err(Into::into)?)
            .map_err(Into::into)
    }

    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut inter) = self.optic1.try_get(source) {
            self.optic2.set(&mut inter, value);
            self.optic1.set(source, inter);
        }
    }
}

impl<O1, O2, S, I, A> Prism<S, A> for ComposedPrism<O1, O2, S, I, A>
where
    O1: Prism<S, I> + Optic<S, I>,
    O2: Prism<I, A> + Optic<I, A>,
    O1::Error: Into<NoFocus>,
    O2::Error: Into<NoFocus>,
{
    fn preview(&self, source: &S) -> Option<A> {
        self.optic2.preview(&self.optic1.preview(source)?)
    }
}
