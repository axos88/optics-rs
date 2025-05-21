use std::convert::Infallible;
use crate::HasGetter;

mod composed;
mod mapped;
mod wrapper;

pub use composed::new as composed_partial_getter;
pub use mapped::new as mapped_partial_getter;
pub use wrapper::PartialGetterImpl;

/// A `PartialGetter` is an optic that focuses on a potential value inside a sum type, providing
/// only a read operations
///
/// It provides:
/// - `try_get` to optionally extract a focus value from a larger type
///
/// This is useful for working with `enum` variants, `Option` values, or
/// other sum types where a focus value might be absent.
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
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the forward mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible one-to-one transformation between two types
pub trait PartialGetter<S, A>: HasGetter<S, A> {}

impl<S, A, PG: HasGetter<S, A>> PartialGetter<S, A> for PG {}



/// Creates a `PartialGetter` that focuses on the entire input.
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
/// A `PartialGetterImpl` instance that implements `PartialGetter<S, S>`
/// and always returns the cloned input value.
///
/// # Example
///
/// ```rust
/// use optics::{identity_partial_getter, HasGetter};
///
/// let getter = identity_partial_getter::<i32>();
/// assert_eq!(getter.try_get(&42), Ok(42));
/// ```
///
/// # See Also
///
/// - [`mapped_partial_getter`] for constructing custom `PartialGetter`s
///   from arbitrary mapping functions.
///
#[must_use]
pub fn identity_partial_getter<S: Clone>()
-> PartialGetterImpl<S, S, impl PartialGetter<S, S, GetterError = Infallible>> {
    mapped_partial_getter(|x: &S| Ok(x.clone()))
}
