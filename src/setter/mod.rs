use core::marker::PhantomData;

pub(crate) mod composed;
pub(crate) mod mapped;
use crate::HasSetter;

pub use composed::new as composed_setter;
pub use mapped::new as mapped_setter;

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

pub struct SetterImpl<S, A, L: Setter<S, A>>(pub L, PhantomData<(S, A)>);

impl<S, A, L: Setter<S, A>> SetterImpl<S, A, L> {
    pub fn new(l: L) -> Self {
        SetterImpl(l, PhantomData)
    }
}

impl<S, A, L: Setter<S, A>> HasSetter<S, A> for SetterImpl<S, A, L> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}

impl<S, A, L: Setter<S, A>> Setter<S, A> for SetterImpl<S, A, L> {}

// Note that a setter cannot be composed with another setter, since we need to be able to retrieve the value that outer optic focuses on to be able to set the value the inner one focuses on.
