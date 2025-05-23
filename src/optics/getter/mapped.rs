use crate::optics::getter::wrapper::GetterImpl;
use crate::{Getter, HasGetter};
use core::convert::Infallible;
use core::marker::PhantomData;

/// Creates a new `Getter` with the provided getter function.
///
/// # Type Parameters
/// - `S`: The source type of the optic
/// - `A`: The target type of the optic
/// # Arguments
///
/// - `get_fn` — A function that retrieves the focus value `A` from the source `S`.
///
/// # Returns
///
/// A new `GetterImpl` instance that can be used as a `Getter<S, A>`.
///
/// # Examples
///
/// ```
/// use optics::{mapped_getter, HasTotalGetter};
///
/// struct Point { x: i32, y: i32 }
/// let x_partial_getter = mapped_getter(
///     |s: &Point| s.x
/// );
///
/// let point = Point { x: 10, y: 20 };
///
/// assert_eq!(x_partial_getter.get(&point), 10);
/// ```
struct MappedGetter<S, A, GET = fn(&S) -> A>
where
    GET: Fn(&S) -> A,
{
    get_fn: GET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, GET> MappedGetter<S, A, GET>
where
    GET: Fn(&S) -> A,
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
    // ```
    /// use `optics::{Lens`, `MappedLens`, Optic};
    ///
    /// struct Point { x: i32, y: i32 }
    /// let mut point = Point { x: 10, y: 20 };
    /// let `x_lens` = `MappedLens::`<Point, `i32>::new`(
    ///     |p| p.x,
    ///     |p, `new_x`| p.x = `new_x`
    /// );
    /// let `x_value` = `x_lens.get(&point)`; // retrieves 10
    /// `x_lens.set(&mut` point, 30); // sets x to 30
    // ```
    ///
    /// # Capturing Closures
    ///
    /// You can also use capturing closures for more flexible behavior, such as when you
    /// need to capture environment variables. In that case, you can specify the trailing
    /// type parameters as `_`, and the compiler will infer them:
    ///
    // ```
    /// use `optics::{Lens`, `MappedLens`, Optic};
    ///
    /// struct Point { x: i32, y: i32 }
    /// let factor = 2;
    /// let mut point = Point { x: 10, y: 20 };
    /// let `x_lens` = `MappedLens::`<Point, i32, _, _>`::new`(
    ///     move |p| p.x * factor,
    ///     move |p, `new_x`| p.x = `new_x` / factor
    /// );
    /// let `x_value` = `x_lens.get(&point)`; // retrieves 10 * 2 = 20
    /// `x_lens.set(&mut` point, 60); // sets x to 60 / 2 = 30
    // ```
    fn new(get_fn: GET) -> Self {
        MappedGetter {
            get_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, GET> HasGetter<S, A> for MappedGetter<S, A, GET>
where
    GET: Fn(&S) -> A,
{
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok((self.get_fn)(source))
    }
}

#[must_use]
pub fn new<S, A, GET>(get_fn: GET) -> GetterImpl<S, A, impl Getter<S, A>>
where
    GET: Fn(&S) -> A,
{
    MappedGetter::new(get_fn).into()
}
