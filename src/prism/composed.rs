use crate::{NoFocus, Optic, Prism, PrismImpl};
use core::marker::PhantomData;

/// A composed `Prism` type, combining two optics into a single prism.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type that results in a `Prism`.
///
/// A `ComposedPrism` not only combines two optics into a single [`Prism`], but it also inherently
/// acts as an `Optic`. This behavior arises from the fact that a `Prism` is itself a
/// more specific form of an optic, and thus any `Prism` composition will also be usable an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedPrism` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Prism`] — the optic type that `ComposedPrism` is based on
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableLens`] — a trait for composing [`Lens`] optics another [`Optic`]
/// - [`crate::composers::ComposablePrism`] — a trait for composing [`Prism`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
pub struct ComposedPrism<O1, O2, S, I, A> {
    optic1: O1,
    optic2: O2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<O1, O2, S, I, A> ComposedPrism<O1, O2, S, I, A> {
    pub(crate) fn new(optic1: O1, optic2: O2) -> Self
    where
        O1: Prism<S, I>,
        O2: Prism<I, A>,
    {
        ComposedPrism {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<O1, O2, S, I, A> Optic<S, A> for ComposedPrism<O1, O2, S, I, A>
where
    O1: Prism<S, I>,
    O2: Prism<I, A>,
{
    type Error = NoFocus;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.preview(source).ok_or(NoFocus)
    }

    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut i) = self.optic1.try_get(source) {
            self.optic2.set(&mut i, value);
            self.optic1.set(source, i);
        }
    }
}

impl<O1, O2, S, I, A> Prism<S, A> for ComposedPrism<O1, O2, S, I, A>
where
    O1: Prism<S, I>,
    O2: Prism<I, A>,
{
    fn preview(&self, source: &S) -> Option<A> {
        self.optic2.preview(&self.optic1.preview(source)?)
    }
}

pub fn new<S, A, I, L1: Prism<S, I>, L2: Prism<I, A>>(
    l1: L1,
    l2: L2,
) -> PrismImpl<S, A, ComposedPrism<L1, L2, S, I, A>> {
    PrismImpl::new(ComposedPrism::new(l1, l2))
}
