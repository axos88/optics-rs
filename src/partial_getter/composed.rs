use crate::partial_getter::wrapper::PartialGetterImpl;
use crate::{HasGetter, PartialGetter};
use core::marker::PhantomData;

struct ComposedPartialGetter<PG1: PartialGetter<S, I>, PG2: PartialGetter<I, A>, E, S, I, A> {
    optic1: PG1,
    optic2: PG2,
    error_fn_1: fn(PG1::GetterError) -> E,
    error_fn_2: fn(PG2::GetterError) -> E,
    _phantom: PhantomData<(S, I, A, E)>,
}

impl<PG1, PG2, E, S, I, A> ComposedPartialGetter<PG1, PG2, E, S, I, A>
where
    PG1: PartialGetter<S, I>,
    PG2: PartialGetter<I, A>,
{
    pub(crate) fn new(
        optic1: PG1,
        optic2: PG2,
        error_fn_1: fn(PG1::GetterError) -> E,
        error_fn_2: fn(PG2::GetterError) -> E,
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

impl<PG1, PG2, E, S, I, A> HasGetter<S, A> for ComposedPartialGetter<PG1, PG2, E, S, I, A>
where
    PG1: PartialGetter<S, I>,
    PG2: PartialGetter<I, A>,
{
    type GetterError = E;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self.optic1.try_get(source).map_err(self.error_fn_1)?;
        self.optic2.try_get(&i).map_err(self.error_fn_2)
    }
}

/// Returns a `PartialGetter` combined from two optics applied one after another.
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

#[must_use]
pub fn new<S, A, I, E, PG1: PartialGetter<S, I>, PG2: PartialGetter<I, A>>(
    l1: PG1,
    l2: PG2,
    error_fn_1: fn(PG1::GetterError) -> E,
    error_fn_2: fn(PG2::GetterError) -> E,
) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = E>> {
    ComposedPartialGetter::new(l1, l2, error_fn_1, error_fn_2).into()
}
