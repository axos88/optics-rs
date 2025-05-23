use crate::{HasGetter, HasSetter};
pub(crate) mod composed;
pub(crate) mod mapped;
mod wrapper;

use crate::HasReverseGet;
pub use composed::new as composed_fallible_iso;
pub use mapped::new as mapped_fallible_iso;
pub use wrapper::FallibleIsoImpl;

/// A `FallibleIso` defines a reversible, but potentially failing conversion between two types.
///
/// It provides:
/// - `try_get` to convert a value of type `S` to type `A`, possibly failing with an error of type `GetterError`
/// - `set` to change the value the optic operates on
/// - `try_reverse_get` to convert a value of type `A` to type `S`, possibly failing with an error of type `ReverseError`
///
/// This is useful when working with a data structure that can be represented in two different ways,
/// but conversion between the two types might fail, such as a String and an `IpAddress`
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
/// - [`Iso`] — a variant of `FallibleIso` where the mapping cannot fail.
/// - [`FallibleIsoImpl`] — the wrapper of opaque struct that implement the `FallibleIso` trait
pub trait FallibleIso<S, A>: HasGetter<S, A> + HasSetter<S, A> + HasReverseGet<S, A> {}

impl<S, A, FI: HasGetter<S, A> + HasSetter<S, A> + HasReverseGet<S, A>> FallibleIso<S, A> for FI {}

/// Creates a `FallibleIso` that maps an input to itself. This is actually an `Iso`.
///
/// It can be useful in cases where you need an identity optic within
/// a composition chain, or as a trivial fallible iso implementation.
///
/// # Type Parameters
///
/// - `S`: The type of the input and output value. Must implement `Clone`.
/// - `GE`: The type of error that can occur during the forward mapping. It's never returned
/// - `RE`: The type of error that can occur during the reverse mapping. It's never returned
///
/// # Returns
///
/// A `FallibleIsoImpl` instance that implements `FallibleIso<S, S>` and always returns the cloned input value.
///
/// # Example
///
/// ```rust
/// use optics::{identity_fallible_iso, HasSetter, HasGetter, HasReverseGet};
///
/// let iso = identity_fallible_iso::<i32, (), ()>();
/// let mut v = 42i32;
///
/// assert_eq!(iso.try_get(&v), Ok(42));
/// iso.set(&mut v, 43);
/// assert_eq!(iso.try_reverse_get(&v), Ok(43));
/// assert_eq!(v, 43);
/// ```
///
/// # See Also
///
/// - [`mapped_fallible_iso`] for constructing custom `FallibleIso`s from arbitrary mapping functions.
#[must_use]
pub fn identity_fallible_iso<S: Clone, GE, RE>()
-> FallibleIsoImpl<S, S, impl FallibleIso<S, S, GetterError = GE, ReverseError = RE>> {
    mapped_fallible_iso(|x: &S| Ok(x.clone()), |x: &S| Ok(x.clone()))
}
