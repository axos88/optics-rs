use crate::optics::iso::Iso;
use crate::optics::iso::wrapper::IsoImpl;
use crate::{HasGetter, HasReverseGet, HasSetter, HasTotalReverseGet};
use core::convert::Infallible;
use core::marker::PhantomData;

/// A concrete implementation of a [`Iso`] between types `S` and `A`.
///
/// /// `IsoImpl` allows the user to create a simple bidirectional, fallible isomorphism by
/// providing two potentially fallible functions: one to convert from `S` to `A`, and one to convert back from `A` to `S`.
///
/// This is the primary way for users to define custom `Iso` optics manually.
///
/// # Type Parameters
/// - `S` — The source type.
/// - `A` — The focus type.
/// - `E` - The error type
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
/// - [`Prism`] — The equivalent for partial optics.
struct MappedIso<S, A, GET = fn(&S) -> A, REV = fn(&A) -> S>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    get_fn: GET,
    rev_fn: REV,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, GET, REV> MappedIso<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    /// Creates a new [`MappedIso`] instance from the provided fallible conversion functions.
    ///
    /// This is the primary way to construct a [`MappedIso`], by supplying two fallible
    /// functions — one for converting from `S` to `A`, and another for converting back from `A` to `S`.
    /// Both conversions may fail, returning a value of type `E`.
    ///
    /// # Arguments
    /// - `get_fn` — A function or closure of type `Fn(&S) -> A` that attempts to extract a
    ///   value of type `A` from a value of type `S`.
    /// - `rev_fn` — A function or closure of type `Fn(&A) -> S` that attempts to reconstruct
    ///   a value of type `S` from a value of type `A`.
    ///
    /// # Returns
    ///
    /// A new `IsoImpl` instance that can be used as a `Iso<S, A>`.
    ///
    /// # Examples
    ///
    // ```
    /// use `optics::IsoImpl`;
    ///
    /// let `fallible_iso` = `IsoImpl::`<i32, String, `String>::new`(
    ///   |i| if *i > 0 { `Ok(i.to_string())` } else { `Err("Negative".to_string())` },
    ///   |s| `s.parse::`<i32>().`map_err(|e`| `e.to_string()`)
    /// );
    // ```
    ///
    /// # Capturing Closures
    ///
    /// You can also use capturing closures for more flexible behavior, such as when you
    /// need to capture environment variables. In that case, you can specify the trailing
    /// type parameters as `_`, and the compiler will infer them:
    ///
    // ```
    /// use optics::IsoImpl;
    ///
    /// let max_value = 100;
    ///
    /// let iso = IsoImpl::<i32, String, _, _, _>::new(
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
    // ```
    pub(crate) fn new(get_fn: GET, rev_fn: REV) -> Self {
        MappedIso {
            get_fn,
            rev_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, GET, REV> HasGetter<S, A> for MappedIso<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok((self.get_fn)(source))
    }
}

impl<S, A, GET, REV> HasSetter<S, A> for MappedIso<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    fn set(&self, source: &mut S, value: A) {
        *source = self.reverse_get(&value);
    }
}

impl<S, A, GET, REV> HasReverseGet<S, A> for MappedIso<S, A, GET, REV>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    type ReverseError = Infallible;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        Ok((self.rev_fn)(value))
    }
}

#[must_use]
pub fn new<S, A, GET, REV>(get_fn: GET, rev_fn: REV) -> IsoImpl<S, A, impl Iso<S, A>>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    MappedIso::new(get_fn, rev_fn).into()
}
