use crate::HasGetter;
use core::convert::Infallible;

/// Provides a simplified interface for optics with infallible getter operations.
///
/// This trait is automatically implemented for any optic that implements
/// [`HasGetter`] with a [`GetterError`] type of [`Infallible`].
///
/// # Example
///
/// ```rust
/// use optics::{HasTotalGetter, mapped_getter};
///
/// struct Point {
///     x: u32,
///     y: u32,
/// }
///
/// let x_getter = mapped_getter(
///     |p: &Point| p.x,
/// );
///
/// let point = Point { x: 10, y: 20 };
/// let x_value = x_getter.get(&point); // x_value is 10
/// ```
///
/// # See also:
///
/// [`HasGetter`]: base trait for optics that provides a potentially fallible getter operation.
pub trait HasTotalGetter<S, A> {
    /// Retrieves a value of type `A` from a source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `source`: A reference to the source of type `S` from which the value is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns the value of type `A` that the optic focuses on.
    fn get(&self, source: &S) -> A;
}

impl<S, A, T> HasTotalGetter<S, A> for T
where
    T: HasGetter<S, A, GetterError = Infallible>,
{
    fn get(&self, source: &S) -> A {
        match self.try_get(source) {
            Ok(value) => value,
        }
    }
}
