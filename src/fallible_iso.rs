use crate::iso::Iso;
use crate::lens::Lens;
use crate::optic::Optic;
use crate::prism::{ComposedPrism, NoFocus, Prism};
use core::marker::PhantomData;

/// A bidirectional, fallible isomorphism between two types `S` and `A`.
///
/// A `FallibleIso` is an optic that provides a potentially lossy, reversible mapping between a
/// source type `S` and a focus type `A`, where **both** the forward (`S → A`) and reverse
/// (`A → S`) transformations can fail independently.
///
/// This makes it suitable for conversions where neither direction is guaranteed to succeed in
/// all cases. Examples include parsing, type coercion, or partial decoding tasks where values
/// may not always be representable in the other form.
///
/// # Supertraits
/// - [`Optic<S, A>`] — provides the primary optic interface for fallible `get` and `set` operations.
/// - [`Prism<S, A>`] — allows using this `FallibleIso` as a `Prism`.
///
/// # Error Semantics
/// The associated `Error` type on the `Optic` supertrait defines the possible error value for
/// both the `try_get` and `try_reverse_get` operations.
///
/// # See Also
/// - [`Iso`] — for total, infallible isomorphisms.
/// - [`Prism`] — for partial optics where only one direction may be partial.
/// - [`Optic`] — the base trait for all optics.
pub trait FallibleIso<S, A>: Optic<S, A> + Prism<S, A>
where
    NoFocus: From<Self::Error>,
{
    /// Attempts to perform the reverse transformation from the focus type `A` back to the source type `S`.
    ///
    /// Since this is a *fallible* isomorphism, the operation may fail if the provided `A` value
    /// cannot be converted back into a valid `S`. The error type is defined by the `Error`
    /// associated type of the [`Optic`] supertrait.
    ///
    /// # Arguments
    /// * `source` — A reference to the focus type value `A`.
    ///
    /// # Returns
    /// `Ok(S)` if the reverse transformation succeeds, or `Err(Self::Error)` if it fails.
    fn try_reverse_get(&self, source: &A) -> Result<S, Self::Error>;
}

/// A concrete implementation of a [`FallibleIso`] between types `S` and `A`.
///
/// /// `FallibleIsoImpl` allows the user to create a simple bidirectional, fallible isomorphism by
/// providing two potentially fallible functions: one to convert from `S` to `A`, and one to convert back from `A` to `S`.
///
/// This is the primary way for users to define custom `FallibleIso` optics manually.
///
/// # Type Parameters
/// - `S` — The source type.
/// - `A` — The focus type.
/// - `E` - The error type
/// - `GET` — The function type used for the `get` operation. Defaults to `fn(&S) -> Result<A, E>`.
/// - `REV` — The function type used for the `reverse_get` operation. Defaults to `fn(&A) -> Result<S, E>`.
///
/// # Construction
///
/// The usual way to construct am `FallibleIsoImpl` is to use `FallibleIsoImpl::<S, A>::new()`, which
/// will use non-capturing closures by default. Alternatively, you can specify the
/// getter and setter type parameters as `FallibleIsoImpl::<S, A,_, _>::new()` for more complex use cases,
/// such as captruring closures.
///
/// # See Also
/// - [`FallibleIso`] — The trait implemented by this struct.
/// - [`Prism`] — The equivalent for partial optics.
pub struct FallibleIsoImpl<S, A, E, GET = fn(&S) -> Result<A, E>, REV = fn(&A) -> Result<S, E>>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    get_fn: GET,
    rev_fn: REV,
    phantom: PhantomData<(S, A, E)>,
}

impl<S, A, E, GET, REV> FallibleIsoImpl<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    /// Creates a new [`FallibleIsoImpl`] instance from the provided fallible conversion functions.
    ///
    /// This is the primary way to construct a [`FallibleIsoImpl`], by supplying two fallible
    /// functions — one for converting from `S` to `A`, and another for converting back from `A` to `S`.
    /// Both conversions may fail, returning a value of type `E`.
    ///
    /// # Arguments
    /// - `get_fn` — A function or closure of type `Fn(&S) -> Result<A, E>` that attempts to extract a
    ///   value of type `A` from a value of type `S`.
    /// - `rev_fn` — A function or closure of type `Fn(&A) -> Result<S, E>` that attempts to reconstruct
    ///   a value of type `S` from a value of type `A`.
    ///
    /// # Returns
    ///
    /// A new `FallibleIsoImpl` instance that can be used as a `FallibleIso<S, A>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use optics::FallibleIsoImpl;
    ///
    /// let fallible_iso = FallibleIsoImpl::<i32, String, String>::new(
    ///   |i| if *i > 0 { Ok(i.to_string()) } else { Err("Negative".to_string()) },
    ///   |s| s.parse::<i32>().map_err(|e| e.to_string())
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
    /// use optics::FallibleIsoImpl;
    ///
    /// let max_value = 100;
    ///
    /// let iso = FallibleIsoImpl::<i32, String, _, _, _>::new(
    ///     move |v| {
    ///         if *v <= max_value {
    ///             Ok(v.to_string())
    ///         } else {
    ///             Err(format!("Value {} exceeds maximum {}", v, max_value))
    ///         }
    ///     },
    ///     move |s| {
    ///         s.parse::<i32>()
    ///             .map_err(|_| format!("Failed to parse '{}'", s))
    ///     },
    /// );
    ///
    /// ```

    pub fn new(get_fn: GET, rev_fn: REV) -> Self {
        FallibleIsoImpl {
            get_fn,
            rev_fn,
            phantom: PhantomData,
        }
    }
}

/// A composed `FallibleIso` type, combining two optics into a single `FallibleIso`.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding ComposableXXX traits, where each optic can be any
/// valid optic type where the result is a `FallibleIso`.
///
/// A `ComposedFallible` not only combines two optics into a single lens, but it also inherently
/// acts as a `Prism` and `Optic`. This behavior arises from the fact that a `FallibleIso` is itself a
/// more specific form of an optic, and prism and thus any `FallibleIso` composition will also be usable as
/// a `Prism` and an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedFallibleIso` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`FallibleIso`] — the core optic type that the `ComposedFallibleIso` is based on
/// - [`Prism`] — the optic type that `ComposedFallibleIso` also acts as
/// - [`Optic`] — the base trait that all optic types implement
pub struct ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
    NoFocus: From<O1::Error> + From<O2::Error>,
{
    optic1: O1,
    optic2: O2,
    _phantom: PhantomData<(S, I, A, E)>,
}

impl<O1, O2, S, I, A, E> ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
    NoFocus: From<O1::Error> + From<O2::Error>,
{
    pub(crate) fn new(optic1: O1, optic2: O2) -> Self where {
        ComposedFallibleIso {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<S, A, E, GET, REV> Optic<S, A> for FallibleIsoImpl<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
    NoFocus: From<E>,
{
    type Error = E;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        (self.get_fn)(source)
    }

    fn set(&self, source: &mut S, value: A) {
        self.try_reverse_get(&value)
            .into_iter()
            .for_each(|s| *source = s)
    }
}

impl<S, A, E, GET, REV> Prism<S, A> for FallibleIsoImpl<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
    NoFocus: From<E>,
{
    fn preview(&self, source: &S) -> Option<A> {
        (self.get_fn)(source).ok()
    }
}

impl<O1, O2, E, S, I, A> Prism<S, A> for ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
    NoFocus: From<O1::Error> + From<O2::Error> + From<E>,
    E: From<O1::Error> + From<O2::Error>,
{
    fn preview(&self, source: &S) -> Option<A> {
        let i = self.optic1.preview(source)?;
        self.optic2.preview(&i)
    }
}

impl<S, A, E, GET, REV> FallibleIso<S, A> for FallibleIsoImpl<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
    NoFocus: From<E>,
{
    fn try_reverse_get(&self, source: &A) -> Result<S, Self::Error> {
        (self.rev_fn)(source)
    }
}

impl<O1, O2, E, S, I, A> FallibleIso<S, A> for ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I> + Optic<S, I>,
    O2: FallibleIso<I, A> + Optic<I, A>,
    E: From<O1::Error> + From<O2::Error>,
    NoFocus: From<O1::Error> + From<O2::Error> + From<E>,
{
    fn try_reverse_get(&self, source: &A) -> Result<S, Self::Error> {
        let i = self.optic2.try_reverse_get(source)?;
        Ok(self.optic1.try_reverse_get(&i)?)
    }
}

/// A trait for composing a `FallibleIso` with other optic types.
///
/// This trait enables the composition of a `FallibleIso` with other types of optics, such as a `Lens`,
/// `Iso`, another `FallibleIso`, or `Prism`.
///
/// # Type Parameters
/// - `S`: The source type that the `FallibleIso` operates on.
/// - `I`: The intermediate type in the optic composition.
/// - `A`: The final target type that the optic composition focuses on.
/// - `O2`: The type of the other optic being composed with this `FallibleIso`.
///
/// # Methods
/// The methods in this trait allow composing a `FallibleIso` with various optic types, returning a composed
/// optic that represents a combined focus on the data. Each method corresponds to a different way of
/// composing optics, resulting in a new type of optic
///
/// # See Also
/// - [`Prism`] — the core trait that defines prism-based optics.
/// - [`Lens`] — a trait for working with lens-based optics, a subset of `Optic`.
/// - [`Iso`] — a trait for reversible transformations between types.
/// - [`FallibleIso`] — a trait for isomorphisms that may fail.
/// - [`crate::ComposedLens`] — a type representing the possible result of composing a lens with other optics
/// - [`ComposedPrism`] — a type representing the possible result of composing a lens with other optics
pub trait ComposableFallibleIso<S, I, A, O2: Optic<I, A>>: FallibleIso<S, I> + Sized
where
    NoFocus: From<Self::Error>,
{
    /// Composes the current `FallibleIso` with an `Lens`.
    ///
    /// This method combines a `FallibleIso` with a `Lens`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Lens` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    fn compose_fallible_iso_with_lens(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Lens<I, A> + Prism<I, A>,
        NoFocus: From<Self::Error> + From<O2::Error>;

    /// Composes the current `FallibleIso` with an `Prism`.
    ///
    /// This method combines a `FallibleIso` with a `Prism`, resulting in a new `Prism`.
    ///
    /// # Requirements
    /// - `other`: The `Prism` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `Prism<S, A>`, which is the resulting composed optic.
    fn compose_fallible_iso_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Prism<I, A>,
        NoFocus: From<Self::Error> + From<O2::Error>;

    /// Composes the current `FallibleIso` with an `FallibleIso`.
    ///
    /// This method combines a `FallibleIso` with a `FallibleIso`, resulting in a new `FallibleIso`.
    ///
    /// # Requirements
    /// - `other`: The `FallibleIso` to compose with the current `FallibleIso`.
    ///
    /// # Returns
    /// - A `FallibleIso<S, A>`, which is the resulting composed optic.
    fn compose_fallible_iso_with_fallible_iso<E>(
        self,
        other: O2,
    ) -> ComposedFallibleIso<Self, O2, E, S, I, A>
    where
        O2: FallibleIso<I, A>,
        E: From<Self::Error> + From<O2::Error>,
        NoFocus: From<O2::Error>;

    /// Composes the current `FallibleIso` with an `Iso`.
    ///
    /// This method combines a `FallibleIso` with a `Iso`, resulting in a new `FallibleIso`.
    ///
    /// # Requirements
    /// - `other`: The `Iso` to compose with the current `Iso`.
    ///
    /// # Returns
    /// - A `FallibleIso<S, A>`, which is the resulting composed optic.
    fn compose_fallible_iso_with_iso(
        self,
        other: O2,
    ) -> ComposedFallibleIso<Self, O2, Self::Error, S, I, A>
    where
        O2: Iso<I, A> + FallibleIso<I, A>;
}

impl<FI, O2, S, I, A> ComposableFallibleIso<S, I, A, O2> for FI
where
    FI: FallibleIso<S, I>,
    O2: Optic<I, A>,
    NoFocus: From<FI::Error>,
{
    fn compose_fallible_iso_with_lens(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Lens<I, A> + Prism<I, A>,
        NoFocus: From<O2::Error> + From<Self::Error>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_fallible_iso_with_prism(self, other: O2) -> ComposedPrism<Self, O2, S, I, A>
    where
        Self: Prism<S, I>,
        O2: Prism<I, A>,
        NoFocus: From<O2::Error> + From<Self::Error>,
    {
        ComposedPrism::new(self, other)
    }

    fn compose_fallible_iso_with_fallible_iso<E>(
        self,
        other: O2,
    ) -> ComposedFallibleIso<Self, O2, E, S, I, A>
    where
        O2: FallibleIso<I, A>,
        E: From<O2::Error>,
        NoFocus: From<O2::Error>,
    {
        ComposedFallibleIso::new(self, other)
    }

    fn compose_fallible_iso_with_iso(
        self,
        other: O2,
    ) -> ComposedFallibleIso<Self, O2, Self::Error, S, I, A>
    where
        O2: Iso<I, A> + FallibleIso<I, A>,
        NoFocus: From<O2::Error>,
    {
        ComposedFallibleIso::new(self, other)
    }
}

impl<O1, O2, S, I, A, E> Optic<S, A> for ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
    E: From<O1::Error> + From<O2::Error>,
    NoFocus: From<O1::Error> + From<O2::Error>,
{
    type Error = E;
    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        Ok(self.optic2.try_get(&self.optic1.try_get(source)?)?)
    }

    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut inter) = self.optic1.try_get(source) {
            self.optic2.set(&mut inter, value);
            self.optic1.set(source, inter);
        }
    }
}
