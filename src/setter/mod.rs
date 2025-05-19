mod composed;
mod mapped;
mod wrapper;

use crate::HasSetter;

pub use composed::new as composed_setter;
pub use mapped::new as mapped_setter;
pub use wrapper::SetterImpl;

/// An optic for focusing on a value that is guaranteed to exist within a larger structure.
///
/// A `Setter` is appropriate for product types (e.g., structs) where the focus is always present.
/// Unlike a [`Prism`], a `Setter` cannot fail to retrieve its focus — hence its associated
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
pub trait Setter<S, A>: HasSetter<S, A> {}

impl<S, A, SETTER: HasSetter<S, A>> Setter<S, A> for SETTER {}

pub fn identity_setter<S, A>() -> SetterImpl<S, A, impl Setter<S, A>> {
    mapped_setter(|_, _| ())
}
