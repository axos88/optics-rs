use crate::optics::partial_getter::wrapper::PartialGetterImpl;
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

/// Creates a `PartialGetter<S,A>` combined from two optics <S, I>, <I, A> applied one after another.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `composable_with_XXX` methods, where the two optics can be of any
/// valid optic type that results in a `PartialGetter`.
///
/// # Type Parameters
/// - `S`: The source type of the first optic
/// - `A`: The target type of the second optic
/// - `I`: The intermediate type: the target type of the first optic and the source type of the second optic
/// - `E`: The common error type for both optics
///
/// # Arguments
/// - `pg1`: The first optic of type `PartialGetter<S, I>`
/// - `pg2`: The second optic of type `PartialGetter<I, A>`
/// - `error_fn_1`: A function that maps the error type of the first optic to a common error type `E`
/// - `error_fn_2`: A function that maps the error type of the second optic to a common error type `E`
///
/// This struct **should not** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `compose_with_XXX` methods on each optic impl.
/// The `ComposedPartialGetter` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`PartialGetter`] — the optic type that `ComposedPartialGetter` is based on
#[must_use]
pub fn new<S, A, I, E, PG1: PartialGetter<S, I>, PG2: PartialGetter<I, A>>(
    pg1: PG1,
    pg2: PG2,
    error_fn_1: fn(PG1::GetterError) -> E,
    error_fn_2: fn(PG2::GetterError) -> E,
) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = E>> {
    ComposedPartialGetter::new(pg1, pg2, error_fn_1, error_fn_2).into()
}
