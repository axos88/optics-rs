use crate::partial_getter::wrapper::PartialGetterImpl;
use crate::{HasPartialGetter, PartialGetter};
use core::marker::PhantomData;

/// A concrete implementation of the [`PartialGetter`] trait.
///
/// This struct allows you to create a `PartialGetter` by providing custom getter and setter functions.
/// It is the primary way to create a `PartialGetter` manually, and is flexible enough to support any
/// getter and setter functions that match the required signatures.
///
/// Typically, you will only need to specify the source type `S` and the focus type `A`.
/// The getter and setter functions will be inferred or explicitly provided.
///
/// If you prefer to use capturing closures, you can specify the trailing type parameters as `_`
/// in the construction of `PartialGetterImpl` to allow the compiler to infer them for you. This is especially
/// useful when you need to use closures with captured environment variables.
///
/// # Construction
///
/// The usual way to construct a `PartialGetterImpl` is to use `PartialGetterImpl::<S, A>::new()`, which
/// will use non-capturing closures by default. Alternatively, you can specify the
/// getter and setter type parameters as `PartialGetterImpl::<S, A,_, _>::new()` for more complex use cases,
/// such as captruring closures.
///
/// # See Also
///
/// - [`Lens`] — a more restrictive optic type for focus values
/// - [`Optic`] — base trait that all optics implement
struct MappedPartialGetter<S, A, E, GET = fn(&S) -> Result<A, E>>
where
    GET: Fn(&S) -> Result<A, E>,
{
    get_fn: GET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, E, GET> MappedPartialGetter<S, A, E, GET>
where
    GET: Fn(&S) -> Result<A, E>,
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
        MappedPartialGetter {
            get_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, E, GET> HasPartialGetter<S, A> for MappedPartialGetter<S, A, E, GET>
where
    GET: Fn(&S) -> Result<A, E>,
{
    type GetterError = E;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        (self.get_fn)(source)
    }
}

#[must_use] pub fn new<S, A, E, GET>(
    get_fn: GET,
) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = E>>
where
    GET: Fn(&S) -> Result<A, E>,
{
    MappedPartialGetter::new(get_fn).into()
}
