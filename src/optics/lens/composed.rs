use crate::HasSetter;
use crate::optics::lens::Lens;
use crate::{HasGetter, HasTotalGetter, LensImpl};
use core::convert::Infallible;
use core::marker::PhantomData;

struct ComposedLens<L1: Lens<S, I>, L2: Lens<I, A>, S, I, A> {
    optic1: L1,
    optic2: L2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<L1, L2, S, I, A> ComposedLens<L1, L2, S, I, A>
where
    L1: Lens<S, I>,
    L2: Lens<I, A>,
{
    fn new(optic1: L1, optic2: L2) -> Self {
        ComposedLens {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<S, I, A, L1, L2> HasGetter<S, A> for ComposedLens<L1, L2, S, I, A>
where
    L1: Lens<S, I>,
    L2: Lens<I, A>,
{
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self.optic1.try_get(source)?;
        self.optic2.try_get(&i)
    }
}

impl<S, I, A, L1, L2> HasSetter<S, A> for ComposedLens<L1, L2, S, I, A>
where
    L1: Lens<S, I>,
    L2: Lens<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        let mut i = self.optic1.get(source);
        self.optic2.set(&mut i, value);
        self.optic1.set(source, i);
    }
}

/// Creates a `Lens<S,A>` combined from two optics <S, I>, <I, A> applied one after another.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `composable_with_XXX` methods, where the two optics can be of any
/// valid optic type that results in a `Lens`.
///
/// # Type Parameters
/// - `S`: The source type of the first optic
/// - `A`: The target type of the second optic
/// - `I`: The intermediate type: the target type of the first optic and the source type of the second optic
///
/// # Arguments
/// - `l1`: The first optic of type `Lens<S, I>`
/// - `l2`: The second optic of type `Lens<I, A>`
///
/// This struct **should not** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `compose_with_XXX` methods on each optic impl.
/// The `ComposedLens` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Lens`] — the optic type that `ComposedLens` is based on
#[must_use]
pub fn new<S, A, I, L1: Lens<S, I>, L2: Lens<I, A>>(
    l1: L1,
    l2: L2,
) -> LensImpl<S, A, impl Lens<S, A>> {
    ComposedLens::new(l1, l2).into()
}
