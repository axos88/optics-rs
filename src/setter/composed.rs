use crate::setter::wrapper::SetterImpl;
use crate::{HasSetter, Prism};
use crate::{Setter};
use core::marker::PhantomData;

struct ComposedSetter<SETTER1: Setter<S, I>, SETTER2: Setter<I, A>, S, I, A> {
    optic1: SETTER1,
    optic2: SETTER2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<SETTER1, SETTER2, S, I, A> ComposedSetter<SETTER1, SETTER2, S, I, A>
where
    SETTER1: Setter<S, I>,
    SETTER2: Setter<I, A>,
{
    pub(self) fn new(optic1: SETTER1, optic2: SETTER2) -> Self {
        ComposedSetter {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<S, I, A, P, SETTER2> HasSetter<S, A> for ComposedSetter<P, SETTER2, S, I, A>
where
    P: Prism<S, I>,
    SETTER2: Setter<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut i) = self.optic1.try_get(source) {
            self.optic2.set(&mut i, value);
            self.optic1.set(source, i);
        }
    }
}

/// A composed `Setter` type, combining two optics into a single `Setter`.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type where the result is a `Setter`.
///
/// A `ComposedSetter` not only combines two optics into a single lens, but it also inherently
/// acts as a `Prism` and `Optic`. This behavior arises from the fact that a `Setter` is itself a
/// more specific form of an optic, and prism and thus any `Setter` composition will also be usable as
/// a `Prism` and an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedSetter` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Setter`] — the core optic type that the `ComposedSetter` is based on
/// - [`Prism`] — the optic type that `ComposedSetter` also acts as
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableSetter`] — a trait for composing [`Setter`] optics another [`Optic`]
/// - [`crate::composers::ComposablePrism`] — a trait for composing [`Prism`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
#[must_use]
pub fn new<S, A, I, P1: Prism<S, I>, SETTER2: Setter<I, A>>(
    l1: P1,
    l2: SETTER2,
) -> SetterImpl<S, A, impl Setter<S, A>> {
    ComposedSetter::new(l1, l2).into()
}
