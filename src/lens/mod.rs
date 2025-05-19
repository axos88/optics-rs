use crate::HasTotalGetter;
use crate::HasGetter;
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
pub trait Lens<S, A>:
    HasGetter<S, A, GetterError = Infallible> + HasSetter<S, A>
{
}

impl<S, A, L: HasGetter<S, A, GetterError = Infallible> + HasSetter<S, A>>
    Lens<S, A> for L
{
}

#[must_use] pub fn identity_lens<S: Clone, E>() -> LensImpl<S, S, impl Lens<S, S>> {
    mapped_lens(|x: &S| x.clone(), |_, _| ())
}
