use crate::{HasGetter, HasReverseGet, HasSetter};
use core::convert::Infallible;

mod composed;
mod mapped;
mod wrapper;

pub use composed::new as composed_iso;
pub use mapped::new as mapped_iso;
pub use wrapper::IsoImpl;

/// An `Iso` defines an isomorphism between two type, which is a bijective, reversible conversion between the members of two types.
///
/// It provides:
/// - `get` to convert a value of type `S` to type `A`
/// - `set` to change the value the optic operates on
/// - `reverse_get` to convert a value of type `A` to type `S`
///
/// This is useful when working with a data structure that can be represented in two different ways,
/// such as a Point in XY coordinates and polar coordinates.
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
/// - [`FallibleIso`] — a variant of `Iso` where the mapping might fail, returning an error
/// - [`IsoImpl`] — the wrapper of opaque struct that implement the `Iso` trait
pub trait Iso<S, A>:
    HasGetter<S, A, GetterError = Infallible>
    + HasSetter<S, A>
    + HasReverseGet<S, A, ReverseError = Infallible>
{
}

impl<
    S,
    A,
    ISO: HasGetter<S, A, GetterError = Infallible>
        + HasSetter<S, A>
        + HasReverseGet<S, A, ReverseError = Infallible>,
> Iso<S, A> for ISO
{
}


/// Creates an `Iso` that maps an input to itself.
///
/// It can be useful in cases where you need an identity optic within
/// a composition chain, or as a trivial iso implementation.
///
/// # Type Parameters
///
/// - `S`: The type of the input and output value. Must implement `Clone`.
///
/// # Returns
///
/// An `IsoImpl` instance that implements `Iso<S, S>`
/// and always returns the cloned input value.
///
/// # Example
///
/// ```rust
/// use optics::{identity_iso, HasSetter, HasTotalGetter, HasTotalReverseGet};
///
/// let iso = identity_iso::<i32>();
/// let mut v = 42i32;
///
/// assert_eq!(iso.get(&v), 42);
/// iso.set(&mut v, 43);
/// assert_eq!(iso.get(&v), 43);
/// assert_eq!(v, 43);
/// ```
///
/// # See Also
///
/// - [`mapped_iso`] for constructing custom `Iso`s from arbitrary mapping functions.
#[must_use]
pub fn identity_iso<S: Clone>() -> IsoImpl<S, S, impl Iso<S, S>> {
    mapped_iso(|x: &S| x.clone(), |x: &S| x.clone())
}
