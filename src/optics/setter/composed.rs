use crate::Setter;
use crate::optics::setter::wrapper::SetterImpl;
use crate::{HasSetter, Prism};
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

/// Creates a `Setter<S,A>` combined from two optics <S, I>, <I, A> applied one after another.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `composable_with_XXX` methods, where the two optics can be of any
/// valid optic type that results in a `PartialGetter`.
///
/// This composer is a bit different from the other optics, as it requires the first optic to also
/// have have a `Getter`, so be a `Prism`, as it requires to read the intermediate value so that it can change its focused value.
///
/// # Type Parameters
/// - `S`: The source type of the first optic, needs to be a `Getter`
/// - `A`: The target type of the second optic
/// - `I`: The intermediate type: the target type of the first optic and the source type of the second optic
///
/// # Arguments
/// - `p1`: The first optic of type `Prism<S, I>`
/// - `s2`: The second optic of type `Setter<I, A>`
///
/// This struct **should not** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `compose_with_XXX` methods on each optic impl.
/// The `ComposedSetter` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Setter`] — the optic type that `ComposedSetter` is based on
#[must_use]
pub fn new<S, A, I, P1: Prism<S, I>, SETTER2: Setter<I, A>>(
    p1: P1,
    s2: SETTER2,
) -> SetterImpl<S, A, impl Setter<S, A>> {
    ComposedSetter::new(p1, s2).into()
}
