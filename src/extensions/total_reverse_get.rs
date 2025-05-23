use crate::HasReverseGet;
use core::convert::Infallible;

/// Provides a simplified interface for optics with infallible reverse-get operations.
///
/// This trait is automatically implemented for any optic that implements
/// [`HasReverseGet`] with a [`ReverseError`] type of [`Infallible`] and
/// [`HasGetter`] with a [`GetterError`] type of [`Infallible`]
///
/// # Example
///
/// ```rust
/// use optics::{HasTotalReverseGet, mapped_iso};
///
/// #[derive(PartialEq, Debug)]
/// struct Point {
///     x: u16,
///     y: u16,
/// }
///
/// let point_iso = mapped_iso(
///     |p: &Point| (p.x as u32) << 16 + p.y as u32,
///     |v| Point { x: (v / (1<<16)) as u16 , y: (v % (1<<16)) as u16 },
/// );
///
/// let point = point_iso.reverse_get(&10);
/// assert_eq!(point, Point { x: 0, y: 10 });
/// ```
///
/// [`HasReverseGet`]: crate::HasReverseGet
/// [`ReverseError`]: crate::HasReverseGet::ReverseError
/// [`Infallible`]: std::convert::Infallible
pub trait HasTotalReverseGet<S, A> {
    /// Reverses a value of type `A` back into a source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `value`: A reference to the value of type `A` to be reversed into the source.
    ///
    /// # Returns
    ///
    /// Returns the source of type `S`.
    fn reverse_get(&self, value: &A) -> S;
}

impl<S, A, T> HasTotalReverseGet<S, A> for T
where
    T: HasReverseGet<S, A, ReverseError = Infallible>,
{
    fn reverse_get(&self, value: &A) -> S {
        match self.try_reverse_get(value) {
            Ok(s) => s,
        }
    }
}
