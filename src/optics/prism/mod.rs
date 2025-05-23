use crate::{mapped_partial_getter, HasGetter};
use crate::HasSetter;
use core::convert::Infallible;

mod composed;
mod mapped;
mod wrapper;

pub use composed::new as composed_prism;
pub use mapped::new as mapped_prism;
pub use wrapper::PrismImpl;

/// A `Prism` is an optic that focuses on a potentially missing value, such as a variant of a
/// sum type (enum).
///
/// It provides:
/// - `try_get` to optionally extract a focus value from a larger type
/// - `set` to set the focused value of a larger type
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
/// - [`Getter`] — an optic that focuses on a potentially missing value in a larger type
/// - [`Setter`] — an optic that can change its focused value
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a required struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible bijective conversion between two types
pub trait Prism<S, A>: HasGetter<S, A> + HasSetter<S, A> {}

impl<S, A, P: HasGetter<S, A> + HasSetter<S, A>> Prism<S, A> for P {}

/// Creates a `Prism` that focuses on the entire input. Note that this is actually a lens in disguise.
///
/// It can be useful in cases where you need an identity optic within
/// a composition chain, or as a trivial prism implementation.
///
/// # Type Parameters
///
/// - `S`: The type of the input and output value. Must implement `Clone`.
///
/// # Returns
///
/// A `PrismImpl` instance that implements `Prism<S, S>`
/// and always returns the cloned input value.
///
/// # Example
///
/// ```rust
/// use optics::{identity_prism, HasGetter, HasSetter};
///
/// let prism = identity_prism::<u32>();
/// let mut v = 42;
/// assert_eq!(prism.try_get(&v), Ok(42));
/// prism.set(&mut v, 43);
/// assert_eq!(prism.try_get(&v), Ok(43));
/// ```
///
/// # See Also
///
/// - [`mapped_prism`] for constructing custom `Prism`s from arbitrary mapping functions.
#[must_use]
pub fn identity_prism<S: Clone>() -> PrismImpl<S, S, impl Prism<S, S, GetterError = Infallible>> {
    mapped_prism(|s: &S| Ok::<_, Infallible>(s.clone()), |s, v| *s = v)
}
