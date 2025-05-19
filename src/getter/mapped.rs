use crate::getter::wrapper::GetterImpl;
use crate::{Getter, HasGetter, HasTotalGetter};
use core::convert::Infallible;
use core::marker::PhantomData;

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
        Ok(self.get(source))
    }
}

#[must_use]
pub fn new<S, A, GET>(get_fn: GET) -> GetterImpl<S, A, impl Getter<S, A>>
where
    GET: Fn(&S) -> A,
{
    MappedGetter::new(get_fn).into()
}
