use crate::{HasPartialGetter, HasSetter};
pub(crate) mod composed;
pub(crate) mod mapped;
mod wrapper;

use crate::HasPartialReversible;
pub use composed::new as composed_fallible_iso;
pub use mapped::new as mapped_fallible_iso;
pub use wrapper::FallibleIsoImpl;

/// A bidirectional, fallible isomorphism between two types `S` and `A`.
///
/// A `FallibleIso` is an optic that provides a potentially lossy, reversible mapping between a
/// source type `S` and a focus type `A`, where **both** the forward (`S → A`) and reverse
/// (`A → S`) transformations can fail independently.
///
/// This makes it suitable for conversions where neither direction is guaranteed to succeed in
/// all cases. Examples include parsing, type coercion, or partial decoding tasks where values
/// may not always be representable in the other form.
///
/// # Supertraits
/// - [`Optic<S, A>`] — provides the primary optic interface for fallible `get` and `set` operations.
/// - [`Prism<S, A>`] — allows using this `FallibleIso` as a `Prism`.
///
/// # Error Semantics
/// The associated `Error` type on the `Optic` supertrait defines the possible error value for
/// both the `try_get` and `try_reverse_get` operations.
///
/// # See Also
/// - [`Iso`] — for total, infallible isomorphisms.
/// - [`Prism`] — for partial optics where only one direction may be partial.
/// - [`Optic`] — the base trait for all optics.
pub trait FallibleIso<S, A>:
    HasPartialGetter<S, A> + HasSetter<S, A> + HasPartialReversible<S, A>
{
}

impl<S, A, FI: HasPartialGetter<S, A> + HasSetter<S, A> + HasPartialReversible<S, A>>
    FallibleIso<S, A> for FI
{
}

#[must_use] pub fn identity_fallible_iso<S: Clone, E>()
-> FallibleIsoImpl<S, S, impl FallibleIso<S, S, GetterError = E, ReverseError = E>> {
    mapped_fallible_iso(|x: &S| Ok(x.clone()), |x: &S| Ok(x.clone()))
}
