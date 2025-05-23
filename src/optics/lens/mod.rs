use crate::{mapped_partial_getter, HasGetter};
use crate::HasSetter;
use core::convert::Infallible;

mod composed;
mod mapped;
mod wrapper;

pub use composed::new as composed_lens;
pub use mapped::new as mapped_lens;
pub use wrapper::LensImpl;

/// An optic for focusing on a value that is guaranteed to exist within a larger structure.
///
/// A `Lens` is appropriate for product types (e.g., structs) where the focus is always present.
/// Unlike a [`Prism`], a `Lens` cannot fail to retrieve its focus — hence its associated
/// [`Optic::Error`] type is fixed to `Infallible`.
///
/// It can also act as a [`Prism`] for compatibility in compositions.
///
/// # See Also
///
/// - [`Optic`] — base trait implemented by all optics
/// - [`Prism`] — optional focus optic for sum types
/// - [`Iso`] — reversible transformations
/// - [`FallibleIso`] — reversible transformations with fallible forward mapping
///
/// A `Lens` is an optic that focuses on a value that is guaranteed to exist within a larger structure.
///
/// A `Lens` is appropriate for product types (e.g., structs) where the focus is always present.
/// Unlike a [`Prism`], a [`Lens`] cannot fail to retrieve its focus — hence its associated
/// [`Getter::GetterError`] type is fixed to `Infallible`.
///
/// It provides:
/// - `get` to extract a focused value from a larger type
/// - `set` to set the focused value of a larger type
///
/// Type Arguments
///   - `S`: The data type the optic operates on
///   - `A`: The data type the optic focuses on
///
/// # Note
///
/// This is a marker trait that is blanket implemented for all structs that satisfy the requirements.
///
/// # See Also
/// - [`Getter`] — an optic that focuses on value that is guaranteed to exist in a larger type
/// - [`Setter`] — an optic that can change its focused value
/// - [`Iso`] — an isomorphism optic representing a reversible bijective conversion between two types
pub trait Lens<S, A>: HasGetter<S, A, GetterError = Infallible> + HasSetter<S, A> {}

impl<S, A, L: HasGetter<S, A, GetterError = Infallible> + HasSetter<S, A>> Lens<S, A> for L {}

/// Creates a `Lens` that focuses on the entire input.
///
/// It can be useful in cases where you need an identity optic within
/// a composition chain, or as a trivial lens implementation.
///
/// # Type Parameters
///
/// - `S`: The type of the input and output value. Must implement `Clone`.
///
/// # Returns
///
/// A `LensImpl` instance that implements `Lens<S, S>`
/// and always returns the cloned input value.
///
/// # Example
///
/// ```rust
/// use optics::{identity_lens, HasTotalGetter, HasSetter};
///
/// let lens = identity_lens::<i32>();
/// let mut v = 42;
/// assert_eq!(lens.get(&v), 42);
/// lens.set(&mut v, 43);
/// assert_eq!(v, 43);
/// ```
///
/// # See Also
///
/// - [`mapped_partial_getter`] for constructing custom `PartialGetter`s
///   from arbitrary mapping functions.
///
#[must_use]
pub fn identity_lens<S: Clone>() -> LensImpl<S, S, impl Lens<S, S>> {
    mapped_lens(|x: &S| x.clone(), |s, v| *s = v)
}
