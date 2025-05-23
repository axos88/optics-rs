use crate::{HasSetter, Setter};
use core::marker::PhantomData;

/// A wrapper of the [`Setter`] optic implementations, encapsulating a setter function.
///
/// `SetterImpl` provides a way to define setters - optics that are able to write to a focued  value
/// of type `A` from a source of type `S`.
/// This struct is particularly useful in scenarios where you need to allow a callee to write into a
/// struct without being able to read its contents.
///
/// # Note
///
/// This struct is not intended to be created by users directly, but it implements a From<Setter<S,A>> so
/// that implementors of new optic types can wrap their concrete implementation of a Setter optic.
///
/// # Type Parameters
///
/// - `S`: The source type from which the value is to be retrieved.
/// - `A`: The target type of the value to be retrieved.
///
/// # See Also
///
/// - [`Setter`] trait for defining custom partial getters.
/// - [`mapped_setter`] function for creating `SetterImpl` instances from mapping functions.
pub struct SetterImpl<S, A, SETTER: Setter<S, A>>(pub SETTER, PhantomData<(S, A)>);

impl<S, A, SETTER: Setter<S, A>> SetterImpl<S, A, SETTER> {
    fn new(l: SETTER) -> Self {
        //TODO: Verify not to nest an Impl inside an Impl - currently seems to be impossible at compile time.
        SetterImpl(l, PhantomData)
    }
}

impl<S, A, SETTER: Setter<S, A>> From<SETTER> for SetterImpl<S, A, SETTER> {
    fn from(value: SETTER) -> Self {
        Self::new(value)
    }
}

impl<S, A, SETTER: Setter<S, A>> HasSetter<S, A> for SetterImpl<S, A, SETTER> {
    fn set(&self, source: &mut S, value: A) {
        self.0.set(source, value);
    }
}
