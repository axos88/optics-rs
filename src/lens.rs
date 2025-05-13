use crate::fallible_iso::FallibleIso;
use crate::iso::Iso;
use crate::optic::Optic;
use crate::prism::{ComposedPrism, NoFocus, Prism};
use core::convert::Infallible;
use core::marker::PhantomData;

/// An optic for focusing on a value that is guaranteed to exist within a larger structure.
///
/// A `Lens` is appropriate for product types (e.g., structs) where the focus is always present.
/// Unlike a [`Prism`], a `Lens` cannot fail to retrieve its focus — hence its associated
/// [`Optic::Error`] type is fixed to `Infallible`.
///
/// It can also act as a [`Prism`] for compatibility in compositions.
///
/// # See Also
///
/// - [`Optic`] — base trait implemented by all optics
/// - [`Prism`] — optional focus optic for sum types
/// - [`Iso`] — reversible transformations
/// - [`FallibleIso`] — reversible transformations with fallible forward mapping
pub trait Lens<S, A>: Optic<S, A, Error = Infallible> + Prism<S, A> {
    /// Retrieves the focus value `A` from the source `S`.
    ///
    /// # Arguments
    ///
    /// - `source` — A reference to the source structure containing the focus.
    ///
    /// # Returns
    ///
    /// The focus value `A`.
    ///
    /// # Behavior
    ///
    /// This operation cannot fail and should not mutate the source.
    ///
    /// # Notes
    ///
    /// This is the infallible counterpart of [`Optic::try_get`], provided because for
    /// lenses the focus is always guaranteed to be present.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use std::net::SocketAddrV4;
    /// use optics::{Lens, LensImpl};
    /// let lens = LensImpl::<SocketAddrV4, u16, _, _>::new(|a| a.port(), |a, p| a.set_port(p));
    ///
    /// let addr = SocketAddrV4::new("127.0.0.1".parse().unwrap(), 8080);
    /// let port = lens.get(&addr);
    ///
    /// assert_eq!(port, 8080u16);
    /// ```
    fn get(&self, source: &S) -> A;
}

/// A concrete implementation of the [`Lens`] trait.
///
/// This struct allows you to create a `Lens` by providing custom getter and setter functions.
/// It is the primary way to create a `Lens` manually, and is flexible enough to support any
/// getter and setter functions that match the required signatures.
///
/// Typically, you will only need to specify the source type `S` and the focus type `A`.
/// The getter and setter functions will be inferred or explicitly provided.
///
/// # Construction
///
/// The usual way to construct a `LensImpl` is to use `LensImpl::<S, A>::new()`, which
/// will use non-capturing closures by default. Alternatively, you can specify the
/// getter and setter type parameters as `LensImpl::<S, A,_, _>::new()`  for more complex use cases,
/// such as captruring closures.
///
/// # See Also
///
/// - [`Lens`] — trait that `LensImpl` implements
/// - [`Prism`] — optional optic type for sum types
/// - [`Optic`] — base trait that all optics implement
pub struct LensImpl<S, A, GET = fn(&S) -> A, SET = fn(&mut S, A)>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    get_fn: GET,
    set_fn: SET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, GET, SET> LensImpl<S, A, GET, SET>
where
    GET: Fn(&S) -> A,
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
        LensImpl {
            get_fn,
            set_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, GET, SET> Optic<S, A> for LensImpl<S, A, GET, SET>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    type Error = Infallible;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        Ok((self.get_fn)(source))
    }

    fn set(&self, source: &mut S, value: A) {
        (self.set_fn)(source, value)
    }
}

impl<S, A, GET, SET> Lens<S, A> for LensImpl<S, A, GET, SET>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    fn get(&self, source: &S) -> A {
        (self.get_fn)(source)
    }
}

impl<S, A, GET, SET> Prism<S, A> for LensImpl<S, A, GET, SET>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    fn preview(&self, source: &S) -> Option<A> {
        Some((self.get_fn)(source))
    }
}

/// A composed `Lens` type, combining two optics into a single `Lens`.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding ComposableXXX traits, where each optic can be any
/// valid optic type where the result is a `Lens`.
///
/// A `ComposedLens` not only combines two optics into a single lens, but it also inherently
/// acts as a `Prism` and `Optic`. This behavior arises from the fact that a `Lens` is itself a
/// more specific form of an optic, and prism and thus any `Lens` composition will also be usable as
/// a `Prism` and an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedLens` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Lens`] — the core optic type that the `ComposedLens` is based on
/// - [`Prism`] — the optic type that `ComposedLens` also acts as
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableLens`] — a trait for composing [`Lens`] optics another [`Optic`]
/// - [`crate::composers::ComposablePrism`] — a trait for composing [`Prism`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
pub struct ComposedLens<O1, O2, S, I, A> {
    optic1: O1,
    optic2: O2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<O1, O2, S, I, A> ComposedLens<O1, O2, S, I, A> {
    pub(crate) fn new(optic1: O1, optic2: O2) -> Self
    where
        O1: Lens<S, I>,
        O2: Lens<I, A>,
    {
        ComposedLens {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

/// A trait for composing a `Lens` with other optic types.
///
/// This trait enables the composition of a `Lens` with other types of optics, such as another `Lens`,
/// `Iso`, `FallibleIso`, or `Prism`.
///
/// # Type Parameters
/// - `S`: The source type that the `Lens` operates on.
/// - `I`: The intermediate type in the optic composition.
/// - `A`: The final target type that the optic composition focuses on.
/// - `O2`: The type of the other optic being composed with this `Lens`.
///
/// # Methods
/// The methods in this trait allow composing a `Lens` with various optic types, returning a composed
/// optic that represents a combined focus on the data. Each method corresponds to a different way of
/// composing optics, resulting in a new type of optic
///
/// # See Also
/// - [`Prism`] — the core trait that defines prism-based optics.
/// - [`Lens`] — a trait for working with lens-based optics, a subset of `Optic`.
/// - [`Iso`] — a trait for reversible transformations between types.
/// - [`FallibleIso`] — a trait for isomorphisms that may fail.
/// - [`ComposedLens`] — a type representing the possible result of composing a lens with other optics
/// - [`ComposedPrism`] — a type representing the possible result of composing a lens with other optics
pub trait ComposableLens<S, I, A, O2: Optic<I, A>>: Lens<S, I> + Sized {
    /// Composes the current `Lens` with a `Lens`.
    ///
    /// This method combines a `Lens` with a `Lens`, resulting in a new `Lens`.
    ///
    /// # Requirements
    /// - `other`: The `Lens` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Lens<S, A>`, which is the resulting composed optic.
    fn compose_lens_with_lens(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    where
        O2: Lens<I, A>;

    /// Composes the current `Lens` with a `Prism`.
    ///
    /// This method combines a `Lens` with a `Prism`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Prism` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    fn compose_lens_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Prism<I, A>,
        NoFocus: From<Self::Error> + From<O2::Error>;

    /// Composes the current `Lens` with a `FallibleIso`.
    ///
    /// This method combines a `Lens` with a `FallibleIso`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `FallibleIso` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    ///
    fn compose_lens_with_fallible_iso(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: FallibleIso<I, A> + Prism<I, A>,
        NoFocus: From<O2::Error>;

    /// Composes the current `Lens` with an `Iso`.
    ///
    /// This method combines a `Lens` with a `Iso`, resulting in a new `Lens`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Lens`.
    ///
    /// # Returns
    /// - A `Lens<S, A>`, which is the resulting composed optic.
    ///
    fn compose_lens_with_iso(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    where
        O2: Iso<I, A> + Lens<I, A>;
}

impl<L, O2, S, I, A> ComposableLens<S, I, A, O2> for L
where
    L: Lens<S, I> + Sized,
    O2: Optic<I, A>,
{
    fn compose_lens_with_lens(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    where
        O2: Lens<I, A>,
    {
        ComposedLens {
            optic1: self,
            optic2: other,
            _phantom: PhantomData,
        }
    }

    fn compose_lens_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Prism<I, A>,
        NoFocus: From<Self::Error> + From<O2::Error>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_lens_with_fallible_iso(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: FallibleIso<I, A> + Prism<I, A>,
        NoFocus: From<O2::Error> + From<Self::Error>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_lens_with_iso(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    where
        O2: Iso<I, A> + Lens<I, A>,
    {
        ComposedLens::new(self, other)
    }
}

impl<O1, O2, S, I, A> Optic<S, A> for ComposedLens<O1, O2, S, I, A>
where
    O1: Optic<S, I, Error = Infallible>,
    O2: Optic<I, A, Error = Infallible>,
{
    type Error = Infallible;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        Ok(self.optic2.try_get(&self.optic1.try_get(source)?)?)
    }

    fn set(&self, source: &mut S, value: A) {
        let Ok(mut inter) = self.optic1.try_get(source);
        self.optic2.set(&mut inter, value);
        self.optic1.set(source, inter);
    }
}

impl<O1, O2, S, I, A> Lens<S, A> for ComposedLens<O1, O2, S, I, A>
where
    O1: Lens<S, I>,
    O2: Lens<I, A>,
{
    fn get(&self, source: &S) -> A {
        self.optic2.get(&self.optic1.get(source))
    }
}

impl<O1, O2, S, I, A> Prism<S, A> for ComposedLens<O1, O2, S, I, A>
where
    O1: Lens<S, I>,
    O2: Lens<I, A>,
{
    fn preview(&self, source: &S) -> Option<A> {
        Some(Lens::<I, A>::get(
            &self.optic2,
            &Lens::<S, I>::get(&self.optic1, source),
        ))
    }
}
