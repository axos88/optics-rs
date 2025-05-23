use crate::optics::iso::Iso;
use crate::optics::iso::wrapper::IsoImpl;
use crate::{HasGetter, HasReverseGet, HasSetter, HasTotalGetter, HasTotalReverseGet, PartialGetter};
use core::convert::Infallible;
use core::marker::PhantomData;

struct ComposedIso<ISO1, ISO2, S, I, A>
where
    ISO1: Iso<S, I>,
    ISO2: Iso<I, A>,
{
    optic1: ISO1,
    optic2: ISO2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<ISO1, ISO2, S, I, A> ComposedIso<ISO1, ISO2, S, I, A>
where
    ISO1: Iso<S, I>,
    ISO2: Iso<I, A>,
{
    pub(crate) fn new(optic1: ISO1, optic2: ISO2) -> Self where {
        ComposedIso {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<ISO1, ISO2, S, I, A> HasGetter<S, A> for ComposedIso<ISO1, ISO2, S, I, A>
where
    ISO1: Iso<S, I>,
    ISO2: Iso<I, A>,
{
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self.optic1.try_get(source)?;
        self.optic2.try_get(&i)
    }
}

impl<ISO1, ISO2, S, I, A> HasSetter<S, A> for ComposedIso<ISO1, ISO2, S, I, A>
where
    ISO1: Iso<S, I>,
    ISO2: Iso<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        let mut i = self.optic1.get(source);
        self.optic2.set(&mut i, value);
        self.optic1.set(source, i);
    }
}

impl<ISO1, ISO2, S, I, A> HasReverseGet<S, A> for ComposedIso<ISO1, ISO2, S, I, A>
where
    ISO1: Iso<S, I>,
    ISO2: Iso<I, A>,
{
    type ReverseError = Infallible;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        Ok(self.reverse_get(value))
    }
}

/// Creates an `Iso<S,A>` combined from two optics <S, I>, <I, A> applied one after another.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `composable_with_XXX` methods, where the two optics can be of any
/// valid optic type that results in a `Iso`.
///
/// # Type Parameters
/// - `S`: The source type of the first optic
/// - `A`: The target type of the second optic
/// - `I`: The intermediate type: the target type of the first optic and the source type of the second optic
///
/// # Arguments
/// - `i1`: The first optic of type `Iso<S, I>`
/// - `i2`: The second optic of type `Iso<I, A>`
///
/// This struct **should not** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `compose_with_XXX` methods on each optic impl.
/// The `ComposedIso` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Iso`] — the optic type that `ComposedIso` is based on
#[must_use]
pub fn new<S, A, I, ISO1: Iso<S, I>, ISO2: Iso<I, A>>(
    i1: ISO1,
    i2: ISO2,
) -> IsoImpl<S, A, impl Iso<S, A>>
where
{
    ComposedIso::new(i1, i2).into()
}
