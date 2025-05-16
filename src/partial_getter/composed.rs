use crate::{HasPartialGetter, PartialGetter};
use core::marker::PhantomData;

/// A composed `PartialGetter` type, combining two optics into a single prism.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type that results in a `PartialGetter`.
///
/// A `ComposedPartialGetter` not only combines two optics into a single [`PartialGetter`], but it also inherently
/// acts as an `Optic`. This behavior arises from the fact that a `PartialGetter` is itself a
/// more specific form of an optic, and thus any `PartialGetter` composition will also be usable an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedPartialGetter` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`PartialGetter`] — the optic type that `ComposedPartialGetter` is based on
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableLens`] — a trait for composing [`Lens`] optics another [`Optic`]
/// - [`crate::composers::ComposablePartialGetter`] — a trait for composing [`PartialGetter`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
pub struct ComposedPartialGetter<O1: PartialGetter<S, I>, O2: PartialGetter<I, A>, E, S, I, A> {
    optic1: O1,
    optic2: O2,
    error_fn_1: fn(O1::GetterError) -> E,
    error_fn_2: fn(O2::GetterError) -> E,
    _phantom: PhantomData<(S, I, A, E)>,
}

impl<O1, O2, E, S, I, A> ComposedPartialGetter<O1, O2, E, S, I, A>
where
    O1: PartialGetter<S, I>,
    O2: PartialGetter<I, A>,
{
    pub(crate) fn new(
        optic1: O1,
        optic2: O2,
        error_fn_1: fn(O1::GetterError) -> E,
        error_fn_2: fn(O2::GetterError) -> E,
    ) -> Self {
        ComposedPartialGetter {
            optic1,
            optic2,
            error_fn_1,
            error_fn_2,
            _phantom: PhantomData,
        }
    }
}

impl<O1, O2, E, S, I, A> HasPartialGetter<S, A> for ComposedPartialGetter<O1, O2, E, S, I, A>
where
    O1: PartialGetter<S, I>,
    O2: PartialGetter<I, A>,
{
    type GetterError = E;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self.optic1.try_get(source).map_err(self.error_fn_1)?;
        self.optic2.try_get(&i).map_err(self.error_fn_2)
    }
}

impl<O1, O2, E, S, I, A> PartialGetter<S, A> for ComposedPartialGetter<O1, O2, E, S, I, A>
where
    O1: PartialGetter<S, I>,
    O2: PartialGetter<I, A>,
{
}

pub fn new<S, A, I, E, L1: PartialGetter<S, I>, L2: PartialGetter<I, A>>(
    l1: L1,
    l2: L2,
    error_fn_1: fn(L1::GetterError) -> E,
    error_fn_2: fn(L2::GetterError) -> E,
) -> ComposedPartialGetter<L1, L2, E, S, I, A> {
    ComposedPartialGetter::new(l1, l2, error_fn_1, error_fn_2)
}
