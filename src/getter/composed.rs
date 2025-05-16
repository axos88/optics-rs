use crate::{Getter, HasGetter};
use core::marker::PhantomData;

/// A composed `Getter` type, combining two optics into a single prism.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type that results in a `Getter`.
///
/// A `ComposedGetter` not only combines two optics into a single [`Getter`], but it also inherently
/// acts as an `Optic`. This behavior arises from the fact that a `Getter` is itself a
/// more specific form of an optic, and thus any `Getter` composition will also be usable an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedGetter` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Getter`] — the optic type that `ComposedGetter` is based on
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableLens`] — a trait for composing [`Lens`] optics another [`Optic`]
/// - [`crate::composers::ComposableGetter`] — a trait for composing [`Getter`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
pub struct ComposedGetter<G1: Getter<S, I>, G2: Getter<I, A>, S, I, A> {
    optic1: G1,
    optic2: G2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<G1, G2, S, I, A> ComposedGetter<G1, G2, S, I, A>
where
    G1: Getter<S, I>,
    G2: Getter<I, A>,
{
    pub(crate) fn new(optic1: G1, optic2: G2) -> Self {
        ComposedGetter {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<G1, G2, S, I, A> HasGetter<S, A> for ComposedGetter<G1, G2, S, I, A>
where
    G1: Getter<S, I>,
    G2: Getter<I, A>,
{
    fn get(&self, source: &S) -> A {
        self.optic2.get(&self.optic1.get(source))
    }
}

impl<G1, G2, S, I, A> Getter<S, A> for ComposedGetter<G1, G2, S, I, A>
where
    G1: Getter<S, I>,
    G2: Getter<I, A>,
{
}

pub fn new<S, A, I, E, L1: Getter<S, I>, L2: Getter<I, A>>(
    l1: L1,
    l2: L2,
) -> ComposedGetter<L1, L2, S, I, A> {
    ComposedGetter::new(l1, l2)
}
