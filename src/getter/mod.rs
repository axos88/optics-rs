mod composed;
mod mapped;
mod wrapper;

use crate::{HasGetter};
pub use composed::new as composed_getter;
use core::convert::Infallible;
pub use mapped::new as mapped_getter;
pub use wrapper::GetterImpl;

/// A `Getter` is an optic that focuses on a potential value inside a sum type.
///
/// It provides:
/// - `preview` to optionally extract a focus value from a larger type
/// - `set` to construct the larger type from a focus value
///
/// This is useful for working with `enum` variants, `Option` values, or
/// other sum types where a focus value might be present.
///
/// Be very careful if you intend to implement this trait yourself, it should not be needed.
///
/// # Note
///
/// `Getter` is currently the most general form of a *concrete optic* in this library.
/// In the future, it may be generalized further to allow any error type in place
/// of the fixed `NoFocus` error. When that happens, `Getter<S, A>` will become a
/// special case of a fully general `Optic<S, A>` abstraction, making this trait
/// redundant and subject to removal in favor of the unified `Optic<S, A>` design.
///
/// This would allow error type specialization while preserving the same core behavior.
///
/// # See Also
/// - [`Optic`] — the base trait for all optic types, potentially unifying `Lens`, `Getter`, and `Iso`
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the forward mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible one-to-one transformation between two types
///
/// - [`NoFocus`] — the current error type returned by `Getter::preview` on failure
pub trait Getter<S, A>: HasGetter<S, A, GetterError = Infallible> {}

impl<S, A, G: HasGetter<S, A, GetterError = Infallible>> Getter<S, A>
    for G
{
}

#[must_use] pub fn identity_getter<S: Clone>() -> GetterImpl<S, S, impl Getter<S, S>> {
    mapped_getter(|x: &S| x.clone())
}
