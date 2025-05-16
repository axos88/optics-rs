use crate::HasSetter;
use crate::{Setter, SetterImpl};
use core::marker::PhantomData;

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
///
pub struct MappedSetter<S, A, SET = fn(&mut S, A)>
where
    SET: Fn(&mut S, A),
{
    set_fn: SET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, SET> MappedSetter<S, A, SET>
where
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
    pub(crate) fn new(set_fn: SET) -> Self {
        MappedSetter {
            set_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, SET> HasSetter<S, A> for MappedSetter<S, A, SET>
where
    SET: Fn(&mut S, A),
{
    fn set(&self, source: &mut S, value: A) {
        (self.set_fn)(source, value);
    }
}

impl<S, A, SET> Setter<S, A> for MappedSetter<S, A, SET> where SET: Fn(&mut S, A) {}

pub fn new<S, A, SET>(set_fn: SET) -> SetterImpl<S, A, MappedSetter<S, A, SET>>
where
    SET: Fn(&mut S, A),
{
    SetterImpl::new(MappedSetter::new(set_fn))
}
