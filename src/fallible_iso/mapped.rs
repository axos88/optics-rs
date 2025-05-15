use crate::fallible_iso::{FallibleIso, FallibleIsoImpl};
use crate::{Optic, Prism};
use core::marker::PhantomData;

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
pub struct MappedFallibleIso<S, A, E, GET = fn(&S) -> Result<A, E>, REV = fn(&A) -> Result<S, E>>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    get_fn: GET,
    rev_fn: REV,
    phantom: PhantomData<(S, A, E)>,
}

impl<S, A, E, GET, REV> MappedFallibleIso<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    /// Creates a new [`MappedFallibleIso`] instance from the provided fallible conversion functions.
    ///
    /// This is the primary way to construct a [`MappedFallibleIso`], by supplying two fallible
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
    pub(crate) fn new(get_fn: GET, rev_fn: REV) -> Self {
        MappedFallibleIso {
            get_fn,
            rev_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, E, GET, REV> Optic<S, A> for MappedFallibleIso<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    type Error = E;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        (self.get_fn)(source)
    }

    fn set(&self, source: &mut S, value: A) {
        self.try_reverse_get(&value)
            .into_iter()
            .for_each(|s| *source = s);
    }
}

impl<S, A, E, GET, REV> Prism<S, A> for MappedFallibleIso<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    fn preview(&self, source: &S) -> Option<A> {
        (self.get_fn)(source).ok()
    }
}
impl<S, A, E, GET, REV> FallibleIso<S, A> for MappedFallibleIso<S, A, E, GET, REV>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    fn try_reverse_get(&self, source: &A) -> Result<S, Self::Error> {
        (self.rev_fn)(source)
    }
}

pub fn new<S, A, E, GET, REV>(
    get_fn: GET,
    rev_fn: REV,
) -> FallibleIsoImpl<S, A, MappedFallibleIso<S, A, E, GET, REV>>
where
    GET: Fn(&S) -> Result<A, E>,
    REV: Fn(&A) -> Result<S, E>,
{
    FallibleIsoImpl::new(MappedFallibleIso::new(get_fn, rev_fn))
}
