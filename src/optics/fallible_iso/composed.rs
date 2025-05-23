use crate::{HasReverseGet, Iso, PartialGetter};
use crate::optics::fallible_iso::FallibleIso;
use crate::optics::fallible_iso::wrapper::FallibleIsoImpl;
use crate::{HasGetter, HasSetter};
use core::marker::PhantomData;

struct ComposedFallibleIso<S, I, A, GE, RE, FI1: FallibleIso<S, I>, FI2: FallibleIso<I, A>>
{
    optic1: FI1,
    optic2: FI2,
    getter_error_fn_1: fn(FI1::GetterError) -> GE,
    getter_error_fn_2: fn(FI2::GetterError) -> GE,
    reverse_error_fn_1: fn(FI1::ReverseError) -> RE,
    reverse_error_fn_2: fn(FI2::ReverseError) -> RE,
    _phantom: PhantomData<(S, I, A, GE, RE)>,
}

impl<S, I, A, GE, RE, FI1: FallibleIso<S, I>, FI2: FallibleIso<I, A>> ComposedFallibleIso<S, I, A, GE, RE, FI1, FI2>
{
    pub(crate) fn new(
        optic1: FI1,
        optic2: FI2,
        getter_error_fn_1: fn(FI1::GetterError) -> GE,
        getter_error_fn_2: fn(FI2::GetterError) -> GE,
        reverse_error_fn_1: fn(FI1::ReverseError) -> RE,
        reverse_error_fn_2: fn(FI2::ReverseError) -> RE,
    ) -> Self where {
        ComposedFallibleIso {
            optic1,
            optic2,
            getter_error_fn_1,
            getter_error_fn_2,
            reverse_error_fn_1,
            reverse_error_fn_2,
            _phantom: PhantomData,
        }
    }
}

impl<S, I, A, GE, RE, FI1: FallibleIso<S, I>, FI2: FallibleIso<I, A>> HasGetter<S, A> for ComposedFallibleIso<S, I, A, GE, RE, FI1, FI2>
{
    type GetterError = GE;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self
            .optic1
            .try_get(source)
            .map_err(self.getter_error_fn_1)?;
        self.optic2.try_get(&i).map_err(self.getter_error_fn_2)
    }
}

impl<S, I, A, GE, RE, FI1: FallibleIso<S, I>, FI2: FallibleIso<I, A>> HasReverseGet<S, A>
    for ComposedFallibleIso<S, I, A, GE, RE, FI1, FI2>
{
    type ReverseError = RE;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        let i = self
            .optic2
            .try_reverse_get(value)
            .map_err(self.reverse_error_fn_2)?;
        self.optic1
            .try_reverse_get(&i)
            .map_err(self.reverse_error_fn_1)
    }
}

impl<S, I, A, GE, RE, FI1: FallibleIso<S, I>, FI2: FallibleIso<I, A>> HasSetter<S, A> for ComposedFallibleIso<S, I, A, GE, RE, FI1, FI2>
{
    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut i) = self.optic1.try_get(source).map_err(self.getter_error_fn_1) {
            self.optic2.set(&mut i, value);
            self.optic1.set(source, i);
        }
    }
}

/// Creates a `FallibleIso<S,A>` combined from two optics <S, I>, <I, A> applied one after another.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `composable_with_XXX` methods, where the two optics can be of any
/// valid optic type that results in a `FallibleIso`.
///
/// # Type Parameters
/// - `S`: The source type of the first optic
/// - `A`: The target type of the second optic
/// - `I`: The intermediate type: the target type of the first optic and the source type of the second optic
/// - `GE`: The common forward mapping error type for both optics
/// - `RE`: The common reverse mapping error type for both optics
///
/// # Arguments
/// - `f1`: The first optic of type `FallibleIso<S, I>`
/// - `f2`: The second optic of type `FallibleIso<I, A>`
/// - `getter_error_fn_1`: A function that maps the forward error type of the first optic to a common error type `GE`
/// - `getter_error_fn_2`: A function that maps the forward error type of the second optic to a common error type `GE`
/// - `reverse_error_fn_1`: A function that maps the reverse error type of the second optic to a common error type `RE`
/// - `reverse_error_fn_2`: A function that maps the reverse error type of the second optic to a common error type `RE`
///
/// This struct **should not** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `compose_with_XXX` methods on each optic impl.
/// The `ComposedFallibleIso` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`FallibleIso`] — the optic type that `ComposedFallibleIso` is based on
#[must_use]
pub fn new<S, A, I, GE, RE, FI1: FallibleIso<S, I>, FI2: FallibleIso<I, A>>(
    f1: FI1,
    f2: FI2,
    getter_error_fn_1: fn(FI1::GetterError) -> GE,
    getter_error_fn_2: fn(FI2::GetterError) -> GE,
    reverse_error_fn_1: fn(FI1::ReverseError) -> RE,
    reverse_error_fn_2: fn(FI2::ReverseError) -> RE,
) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A, GetterError = GE, ReverseError = RE>>
where
{
    FallibleIsoImpl::new(ComposedFallibleIso::new(
        f1,
        f2,
        getter_error_fn_1,
        getter_error_fn_2,
        reverse_error_fn_1,
        reverse_error_fn_2,
    ))
}
