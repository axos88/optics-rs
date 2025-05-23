mod composed;
mod mapped;
mod wrapper;

use crate::{mapped_partial_getter, HasGetter};
pub use composed::new as composed_getter;
use core::convert::Infallible;
pub use mapped::new as mapped_getter;
pub use wrapper::GetterImpl;

/// A `Getter` is an optic that focuses on a value inside a product type.
///
/// It provides:
/// - `get` to extract a focused value from a larger type
///
/// This is useful for working for example with required fields of a struct
///
/// Type Arguments:
///   - `S`: The data type the optic operates on
///   - `A`: The data type the optic focuses on
///
/// # Note
///
/// This is a marker trait that is blanket implemented for all structs that satisfy the requirements.
///
/// # See Also
/// - [`HasGetter`] - A base trait for optics that provides a partial getter operation.
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a struct field)
/// - [`Iso`] — an isomorphism optic representing a reversible one-to-one transformation between two types
pub trait Getter<S, A>: HasGetter<S, A, GetterError = Infallible> {}

impl<S, A, G: HasGetter<S, A, GetterError = Infallible>> Getter<S, A> for G {}



/// Creates a `Getter` that focuses on the entire input.
///
/// It can be useful in cases where you need an identity optic within
/// a composition chain, or as a trivial getter implementation.
///
/// # Type Parameters
///
/// - `S`: The type of the input and output value. Must implement `Clone`.
///
/// # Returns
///
/// A `GetterImpl` instance that implements `Getter<S, S>`
/// and always returns the cloned input value.
///
/// # Example
///
/// ```rust
/// use optics::{identity_getter, HasTotalGetter};
///
/// let getter = identity_getter::<i32>();
/// assert_eq!(getter.get(&42), 42);
/// ```
///
/// # See Also
///
/// - [`mapped_getter`] for constructing custom `Getter`s
///   from an arbitrary mapping function.
///
#[must_use]
pub fn identity_getter<S: Clone>() -> GetterImpl<S, S, impl Getter<S, S>> {
    mapped_getter(|x: &S| x.clone())
}
