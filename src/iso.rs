use crate::fallible_iso::{ComposedFallibleIso, FallibleIso};
use crate::lens::{ComposedLens, Lens};
use crate::optic::Optic;
use crate::prism::{ComposedPrism, NoFocus, Prism};
use core::convert::Infallible;
use core::marker::PhantomData;

/// An isomorphism between two types `S` and `A`.
///
/// An `Iso` is a bidirectional optic that provides a one-to-one, lossless, and reversible mapping
/// between a source type `S` and a focus type `A`. Unlike general `Prism` or `Lens` optics, an
/// `Iso` guarantees that for every `S` there exists exactly one corresponding `A`, and vice versa.
///
/// Because it is total and invertible, an `Iso` is both a `Lens` and a `Prism`, as well as a
/// `FallibleIso` with an infallible error type (`Infallible`). This means it participates fully
/// in the optic hierarchy, providing total reads, total writes, and reversible transformations.
///
/// # Supertraits
/// - [`Optic<S, A, Error = Infallible>`] — ensures that operations on the `Iso` cannot fail.
/// - [`FallibleIso<S, A>`] — provides the fallible isomorphism API, but with `Infallible` error.
/// - [`Prism<S, A>`] — allows using this `Iso` as a `Prism`.
/// - [`Lens<S, A>`] — allows using this `Iso` as a `Lens`.
///
/// # See Also
/// - [`Lens`] — for total, read/write optics.
/// - [`Prism`] — for partial optics.
/// - [`FallibleIso`] — for reversible optics that can fail.
/// - [`Optic`] — the base trait for all optics.
pub trait Iso<S, A>:
    Optic<S, A, Error = Infallible> + FallibleIso<S, A> + Prism<S, A> + Lens<S, A>
{
    /// Performs the reverse transformation from the focus type `A` back to the source type `S`.
    ///
    /// Since an `Iso` guarantees a total, bijective mapping, this method must always succeed.
    ///
    /// # Arguments
    /// * `source` — A reference to the focus type value `A`.
    ///
    /// # Returns
    /// The corresponding source type value `S`.
    fn reverse_get(&self, source: &A) -> S;
}

/// A concrete implementation of an [`Iso`] between types `S` and `A`.
///
/// `IsoImpl` allows the user to create a simple isomorphism by providing two total, infallible
/// functions: one to convert from `S` to `A`, and one to convert back from `A` to `S`.
///
/// This is the primary way for users to define custom `Iso` optics manually.
///
/// # Type Parameters
/// - `S` — The source type.
/// - `A` — The focus type.
/// - `GET` — The function type used for the `get` operation. Defaults to `fn(&S) -> A`.
/// - `REV` — The function type used for the `reverse_get` operation. Defaults to `fn(&A) -> S`.
///
/// # Construction
///
/// The usual way to construct am `IsoImpl` is to use `IsoImpl::<S, A>::new()`, which
/// will use non-capturing closures by default. Alternatively, you can specify the
/// getter and setter type parameters as `IsoImpl::<S, A,_, _>::new()` for more complex use cases,
/// such as captruring closures.
///
/// # See Also
/// - [`Iso`] — The trait implemented by this struct.
/// - [`FallibleIso`] — The equivalent for fallible optics.
/// - [`Lens`] — The equivalent for total read/write optics.
/// - [`Prism`] — The equivalent for partial optics.
pub struct IsoImpl<S, A, GET = fn(&S) -> A, REV = fn(&A) -> S>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    get_fn: GET,
    rev_fn: REV,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, GET, REV> IsoImpl<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    /// Creates a new [`IsoImpl`] instance from the provided total conversion functions.
    ///
    /// This is the primary way to construct an [`IsoImpl`], by supplying two total,
    /// infallible functions — one for converting from `S` to `A`, and another for
    /// converting back from `A` to `S`.
    ///
    /// # Arguments
    /// - `get_fn` — A function or closure of type `Fn(&S) -> A` that extracts a value of type `A`
    ///   from a value of type `S`.
    /// - `rev_fn` — A function or closure of type `Fn(&A) -> S` that reconstructs a value of type `S`
    ///   from a value of type `A`.
    ///
    /// # Returns
    ///
    /// A new `IsoImpl` instance that can be used as an `Iso<S, A>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use optics::IsoImpl;
    /// let iso = IsoImpl::<i32, String>::new(
    ///     |s| s.to_string(),
    ///     |a| a.parse().unwrap(),
    /// );
    /// ```
    ///
    /// # Capturing Closures
    ///
    /// You can also use capturing closures for more flexible behavior, such as when you
    /// need to capture environment variables. In that case, you can specify the trailing
    /// type parameters as `_`, and the compiler will infer them:
    ///
    /// ```
    /// use optics::IsoImpl;
    /// let suffix = " units".to_string();
    /// let suffix2 = suffix.clone();
    /// let iso = IsoImpl::<i32, String, _, _>::new(
    ///     move |s| format!("{s}{}", suffix),
    ///     move |a| a.trim_end_matches(&suffix2).parse().unwrap(),
    /// );
    /// ```
    pub fn new(get_fn: GET, rev_fn: REV) -> Self {
        IsoImpl {
            get_fn,
            rev_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, GET, REV> Optic<S, A> for IsoImpl<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    type Error = Infallible;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        Ok((self.get_fn)(source))
    }

    fn set(&self, source: &mut S, value: A) {
        *source = self.reverse_get(&value);
    }
}

impl<S, A, GET, REV> Lens<S, A> for IsoImpl<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    fn get(&self, source: &S) -> A {
        (self.get_fn)(source)
    }
}

impl<S, A, GET, REV> Prism<S, A> for IsoImpl<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    fn preview(&self, source: &S) -> Option<A> {
        Some((self.get_fn)(source))
    }
}

/// A composed [`Iso`] type, combining two optics into a single [`Iso`].
///
/// This struct is automatically created by composing two existing optics into a more general `Iso`.
/// It is **not** intended to be directly constructed outside the crate, but is generated through
/// the composition of two optics via the appropriate `ComposableXXX` traits.
///
/// A `ComposedIso` not only combines two optics into a single lens, but it also inherently
/// acts as a `FallibleIso`, `Lens`, a `Prism` and an `Optic`. This behavior arises from the fact
/// that a `Iso` is itself a more specific form of an optic, prism, lens and fallible iso and thus
/// any `Iso` composition will also be usable as a `FallibleIso`, `Lens`, `Prism` or `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the corresponding `ComposableXXX` trait for each optic type.
/// The `ComposedIso` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Lens`] — the optic type that `ComposedIso` also acts as
/// - [`Prism`] — the optic type that `ComposedIso` also acts as
/// - [`FallibleIso`] — the optic type that `ComposedIso` also acts as
/// - [`Iso`] — the optic type that `ComposedIso` is based on
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableLens`] — a trait for composing [`Lens`] optics another [`Optic`]
/// - [`crate::composers::ComposablePrism`] — a trait for composing [`Prism`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
pub struct ComposedIso<O1, O2, S, I, A> {
    optic1: O1,
    optic2: O2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<O1, O2, S, I, A> ComposedIso<O1, O2, S, I, A> {
    pub(crate) fn new(optic1: O1, optic2: O2) -> Self
    where
        O1: Iso<S, I>,
        O2: Iso<I, A>,
    {
        ComposedIso {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<S, A, GET, REV> Iso<S, A> for IsoImpl<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    fn reverse_get(&self, source: &A) -> S {
        (self.rev_fn)(source)
    }
}

impl<S, A, GET, REV> FallibleIso<S, A> for IsoImpl<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    fn try_reverse_get(&self, source: &A) -> Result<S, Infallible> {
        Ok((self.rev_fn)(source))
    }
}

impl<O1, O2, S, I, A> Iso<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
    O1::Error: Into<NoFocus>,
    O2::Error: Into<NoFocus>,
{
    fn reverse_get(&self, source: &A) -> S {
        let i = self.optic2.reverse_get(source);
        self.optic1.reverse_get(&i)
    }
}

impl<O1, O2, S, I, A> FallibleIso<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
    O1::Error: Into<NoFocus>,
    O2::Error: Into<NoFocus>,
{
    fn try_reverse_get(&self, source: &A) -> Result<S, Infallible> {
        let i = self.optic2.try_reverse_get(source)?;
        self.optic1.try_reverse_get(&i)
    }
}

impl<O1, O2, S, I, A> Prism<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
    O1::Error: Into<NoFocus>,
    O2::Error: Into<NoFocus>,
{
    fn preview(&self, source: &S) -> Option<A> {
        let i = self.optic1.preview(source)?;
        self.optic2.preview(&i)
    }
}

impl<O1, O2, S, I, A> Lens<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
    O1::Error: Into<NoFocus>,
    O2::Error: Into<NoFocus>,
{
    fn get(&self, source: &S) -> A {
        let i = self.optic1.get(source);
        self.optic2.get(&i)
    }
}

/// A trait for composing an `Iso` with other optic types.
///
/// This trait enables the composition of an `Iso` with other types of optics, such as a `Lens`,
/// another `Iso`, `FallibleIso`, or `Prism`.
///
/// # Type Parameters
/// - `S`: The source type that the `Iso` operates on.
/// - `I`: The intermediate type in the optic composition.
/// - `A`: The final target type that the optic composition focuses on.
/// - `O2`: The type of the other optic being composed with this `Iso`.
///
/// # Methods
/// The methods in this trait allow composing an `Iso` with various optic types, returning a composed
/// optic that represents a combined focus on the data. Each method corresponds to a different way of
/// composing optics, resulting in a new type of optic
///
/// # See Also
/// - [`Prism`] — the core trait that defines prism-based optics.
/// - [`Lens`] — a trait for working with lens-based optics, a subset of `Optic`.
/// - [`Iso`] — a trait for reversible transformations between types.
/// - [`FallibleIso`] — a trait for isomorphisms that may fail.
pub trait ComposableIso<S, I, A, O2: Optic<I, A>>: Iso<S, I> + Sized {
    /// Composes the current `Iso` with an `Lens`.
    ///
    /// This method combines a `Iso` with a `Lens`, resulting in a new `Lens`.
    ///
    /// # Requirements
    /// - `other`: The `Lens` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `Lens<S, A>`, which is the resulting composed optic.
    ///
    fn compose_iso_with_lens(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    where
        O2: Lens<I, A>,
        Self: Lens<S, I>;

    /// Composes the current `Iso` with an `Prism`.
    ///
    /// This method combines an `Iso` with a `Prism`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    fn compose_iso_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Prism<I, A>,
        Self::Error: Into<NoFocus>,
        O2::Error: Into<NoFocus>;

    /// Composes the current `Iso` with an `FallibleIso`.
    ///
    /// This method combines an `Iso` with a `FallibleIso`, resulting in a new `FallibleIso`.
    ///
    /// # Requirements
    /// - `other`: The `FallibleIso` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `FallibleIso<S, A>`, which is the resulting composed optic.
    fn compose_iso_with_fallible_iso<E>(
        self,
        other: O2,
    ) -> ComposedFallibleIso<Self, O2, E, S, I, A>
    where
        Self: FallibleIso<S, I>,
        O2: FallibleIso<I, A>,
        E: From<O2::Error> + From<Self::Error>,
        O2::Error: Into<NoFocus>;

    /// Composes the current `Iso` with an `Iso`.
    ///
    /// This method combines an `Iso` with a `Iso`, resulting in a new `Iso`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `Iso<S, A>`, which is the resulting composed optic.
    fn compose_iso_with_iso(self, other: O2) -> ComposedIso<Self, O2, S, I, A>
    where
        O2: Iso<I, A>;
}

impl<ISO, O2, S, I, A> ComposableIso<S, I, A, O2> for ISO
where
    ISO: Iso<S, I>,
    O2: Optic<I, A>,
{
    fn compose_iso_with_lens(self, other: O2) -> ComposedLens<Self, O2, S, I, A>
    where
        Self: Lens<S, I>,
        O2: Lens<I, A>,
        O2::Error: Into<NoFocus>,
    {
        ComposedLens::new(self, other)
    }

    fn compose_iso_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Prism<I, A>,
        Self::Error: Into<NoFocus>,
        O2::Error: Into<NoFocus>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_iso_with_fallible_iso<E>(
        self,
        other: O2,
    ) -> ComposedFallibleIso<Self, O2, E, S, I, A>
    where
        Self: FallibleIso<S, I>,
        O2: FallibleIso<I, A>,
        E: From<O2::Error> + From<Self::Error>,
        O2::Error: Into<NoFocus>,
        Self::Error: Into<NoFocus>,
    {
        ComposedFallibleIso::new(self, other)
    }

    fn compose_iso_with_iso(self, other: O2) -> ComposedIso<Self, O2, S, I, A>
    where
        O2: Iso<I, A>,
    {
        ComposedIso::new(self, other)
    }
}

impl<O1, O2, S, I, A> Optic<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    type Error = Infallible;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.optic2.try_get(&self.optic1.try_get(source)?)
    }

    fn set(&self, source: &mut S, value: A) {
        let Ok(mut inter) = self.optic1.try_get(source);
        self.optic2.set(&mut inter, value);
        self.optic1.set(source, inter);
    }
}
