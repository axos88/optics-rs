use crate::optics::getter::wrapper::GetterImpl;
use crate::{Getter, HasGetter, HasTotalGetter};
use core::convert::Infallible;
use core::marker::PhantomData;

struct ComposedGetter<G1: Getter<S, I>, G2: Getter<I, A>, S, I, A> {
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
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.optic2.get(&self.optic1.get(source)))
    }
}

/// Creates a `Getter<S,A>` combined from two optics <S, I>, <I, A> applied one after another.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `compose_with_XXX` methods, where the two optics can be of any
/// valid optic type that results in a `Getter`.
///
/// # Type Parameters
/// - `S`: The source type of the first optic
/// - `A`: The target type of the second optic
/// - `I`: The intermediate type: the target type of the first optic and the source type of the second optic
///
/// # Arguments
/// - `g1`: The first optic of type `Getter<S, I>`
/// - `g2`: The second optic of type `Getter<I, A>`
///
/// This struct **should not** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `compose_with_XXX` methods on each optic impl.
/// The `ComposedGetter` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Getter`] — the optic type that `ComposedGetter` is based on
#[must_use]
pub fn new<S, A, I, G1: Getter<S, I>, G2: Getter<I, A>>(
    l1: G1,
    l2: G2,
) -> GetterImpl<S, A, impl Getter<S, A>> {
    ComposedGetter::new(l1, l2).into()
}
