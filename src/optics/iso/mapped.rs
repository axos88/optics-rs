use crate::optics::iso::Iso;
use crate::optics::iso::wrapper::IsoImpl;
use crate::{HasGetter, HasReverseGet, HasSetter, HasTotalReverseGet};
use core::convert::Infallible;
use core::marker::PhantomData;

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
    fn new(get_fn: GET, rev_fn: REV) -> Self {
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

/// Creates a new `Iso` with the provided mapping functions.
///
/// # Type Parameters
/// - `S`: The source type of the optic
/// - `A`: The target type of the optic
/// # Arguments
///
/// - `get_fn` — A function that converts a value of type `S` to a value of type `A`.
/// - `rev_fn` — A function that converts a value of type `A` to a value of type `S`.
///
/// # Returns
///
/// A new `IsoImpl` instance that can be used as a `Iso<S, A>`.
///
/// # Examples
///
/// ```
/// use optics::{mapped_iso, HasSetter, HasTotalGetter, HasTotalReverseGet};
///
/// #[derive(Debug)]
/// struct Cartesan { x: f64, y: f64 }
///
/// const EPSILON: f64 = 1e-10;
///
/// impl PartialEq for Cartesan {
///     fn eq(&self, other: &Self) -> bool {
///         (self.x - other.x).abs() < EPSILON &&
///         (self.y - other.y).abs() < EPSILON
///     }
/// }
///
/// #[derive(Debug)]
/// struct Polar { r: f64, theta: f64 }
///
/// impl PartialEq for Polar {
///     fn eq(&self, other: &Self) -> bool {
///         (self.r - other.r).abs() < EPSILON &&
///         (self.theta - other.theta).abs() < EPSILON
///     }
///}
///
/// let cartesan_polar_iso = mapped_iso(
///     |c: &Cartesan| {
///       let r = (c.x.powi(2) + c.y.powi(2)).sqrt();
///       let theta = c.y.atan2(c.x);
///       Polar { r, theta }
///     },
///     |p: &Polar| Cartesan { x: p.r * p.theta.cos(), y: p.r * p.theta.sin() }
/// );
///
/// let mut cartesan = Cartesan { x: 3.0, y: 4.0 };
/// let mut polar = Polar { r: 5.0, theta: 0.9272952180016122 };
///
/// assert_eq!(cartesan_polar_iso.get(&cartesan), polar);
/// assert_eq!(cartesan_polar_iso.reverse_get(&polar), cartesan);
/// cartesan_polar_iso.set(&mut cartesan, Polar { r: 10.0, theta: 0.9272952180016122 });
/// assert_eq!(cartesan, Cartesan { x: 6.0, y: 8.0 });
/// ```
#[must_use]
pub fn new<S, A, GET, REV>(get_fn: GET, rev_fn: REV) -> IsoImpl<S, A, impl Iso<S, A>>
where
    GET: Fn(&S) -> A,
    REV: Fn(&A) -> S,
{
    MappedIso::new(get_fn, rev_fn).into()
}
