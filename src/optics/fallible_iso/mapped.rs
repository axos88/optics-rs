use crate::optics::fallible_iso::wrapper::FallibleIsoImpl;
use crate::{FallibleIso, HasReverseGet};
use crate::{HasGetter, HasSetter};
use core::marker::PhantomData;

struct MappedFallibleIso<S, A, GE, RE, GET = fn(&S) -> Result<A, GE>, REV = fn(&A) -> Result<S, RE>>
where
    GET: Fn(&S) -> Result<A, GE>,
    REV: Fn(&A) -> Result<S, RE>,
{
    get_fn: GET,
    rev_fn: REV,
    phantom: PhantomData<(S, A, GE, RE)>,
}

impl<S, A, GE, RE, GET, REV> MappedFallibleIso<S, A, GE, RE, GET, REV>
where
    GET: Fn(&S) -> Result<A, GE>,
    REV: Fn(&A) -> Result<S, RE>,
{
    fn new(get_fn: GET, rev_fn: REV) -> Self {
        MappedFallibleIso {
            get_fn,
            rev_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, GE, RE, GET, REV> HasGetter<S, A> for MappedFallibleIso<S, A, GE, RE, GET, REV>
where
    GET: Fn(&S) -> Result<A, GE>,
    REV: Fn(&A) -> Result<S, RE>,
{
    type GetterError = GE;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        (self.get_fn)(source)
    }
}

impl<S, A, GE, RE, GET, REV> HasSetter<S, A> for MappedFallibleIso<S, A, GE, RE, GET, REV>
where
    GET: Fn(&S) -> Result<A, GE>,
    REV: Fn(&A) -> Result<S, RE>,
{
    fn set(&self, source: &mut S, value: A) {
        self.try_reverse_get(&value)
            .into_iter()
            .for_each(|s| *source = s);
    }
}

impl<S, A, GE, RE, GET, REV> HasReverseGet<S, A> for MappedFallibleIso<S, A, GE, RE, GET, REV>
where
    GET: Fn(&S) -> Result<A, GE>,
    REV: Fn(&A) -> Result<S, RE>,
{
    type ReverseError = RE;

    fn try_reverse_get(&self, source: &A) -> Result<S, Self::ReverseError> {
        (self.rev_fn)(source)
    }
}

/// Creates a new `FallibleIso` with the provided getter function.
///
/// # Type Parameters
/// - `S`: The source type of the optic
/// - `A`: The target type of the optic
/// - `GE`: The error type returned when the forward mapping fails
/// - `RE`: The error type returned when the reverse mapping fails
/// # Arguments
///
/// - `get_fn` — A function that faillibly maps the value of type `S` to a value of type `A`.
/// - `rev_fn` — A function that faillibly maps the value of type `A` back to a value of type `S`.
///
/// # Returns
///
/// A new `FallibleIsoImpl` instance that can be used as a `FallibleIso<S, A>`.
///
/// # Examples
///
/// ```
/// use std::net::IpAddr;
/// use optics::{mapped_fallible_iso, HasGetter, HasReverseGet, HasSetter};
///
/// fn s2port(s: &String) -> Result<u16, ()> {
///     match s.parse::<u16>() {
///         Ok(n) if n > 0 => Ok(n),
///         _ => Err(()),
///     }
/// }
///
/// fn port2s(port: &u16) -> Result<String, ()> {
///     if *port > 0 {
///         Ok(port.to_string())
///     } else {
///         Err(())
///     }
/// }
/// let string_to_port = mapped_fallible_iso(s2port, port2s);
///
/// let mut s = "8081".to_string();
///
/// assert_eq!(string_to_port.try_get(&s), Ok(8081));
/// assert_eq!(string_to_port.try_reverse_get(&8081), Ok("8081".to_string()));
/// string_to_port.set(&mut s, 8082u16);
/// assert_eq!(s, "8082".to_string());
/// ```
#[must_use]
pub fn new<S, A, GE, RE, GET, REV>(
    get_fn: GET,
    rev_fn: REV,
) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A, GetterError = GE, ReverseError = RE>>
where
    GET: Fn(&S) -> Result<A, GE>,
    REV: Fn(&A) -> Result<S, RE>,
{
    MappedFallibleIso::new(get_fn, rev_fn).into()
}
