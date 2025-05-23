use crate::{HasGetter, HasSetter};

/// Provides a convenient interface for applying a transformation function over a target value within a source.
///
/// This trait is automatically implemented for any optic that implements
/// [`HasGetter`] and [`HasSetter`].
///
/// # Example
///
/// ```rust
/// use optics::{Lens, HasOver, mapped_prism, mapped_lens};
///
/// struct Point {
///     x: u32,
///     y: u32,
/// }
///
/// let x_lens = mapped_lens(
///     |p: &Point| p.x,
///     |p: &mut Point, x| { p.x = x },
/// );
///
/// let mut point = Point { x: 10, y: 20 };
/// x_lens.over(&mut point, |x| x + 5);
/// assert_eq!(point.x, 15);
/// ```
///
/// # See also:
///
/// [`HasGetter`]: crate::HasGetter
/// [`GetterError`]: crate::HasGetter::GetterError
/// [`Infallible`]: std::convert::Infallible
/// [`HasSetter`]: crate::HasSetter
pub trait HasOver<S, A> {
    /// Retrieves a value of type `A` from a source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `source`: A reference to the source of type `S` from which the value is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns the value of type `A` that the optic focuses on.
    fn over<F>(&self, source: &mut S, f: F)
    where
        F: Fn(A) -> A;
}

impl<S, A, T> HasOver<S, A> for T
where
    T: HasGetter<S, A> + HasSetter<S, A>,
{
    fn over<F>(&self, source: &mut S, f: F)
    where
        F: Fn(A) -> A,
    {
        if let Ok(value) = self.try_get(source) {
            self.set(source, f(value));
        }
    }
}
