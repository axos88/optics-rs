use crate::HasGetter;
use crate::HasSetter;
use crate::optics::prism::Prism;
use crate::optics::prism::wrapper::PrismImpl;
use core::marker::PhantomData;

/// A `ComposedPrism` represents the composition of two optics, resulting in a `Prism` that focuses
/// from a source type `S` to a target type `A` through an intermediate type `I`.
///
/// This struct enables the composition of various combinations of optics, such as:
/// - `Prism` + `Prism`
/// - `Prism` + `Lens`
/// - `FallibleIso` + `Lens`
///
/// The composition handles the potential fallibility of each optic by providing error mapping
/// functions to unify different error types into a single error type `E`.
///
/// # Type Parameters
/// - `O1`: The first optic, focusing from `S` to `I`.
/// - `O2`: The second optic, focusing from `I` to `A`.
/// - `E`: The unified error type resulting from composing the getter errors of `O1` and `O2`.
/// - `S`: The source type.
/// - `I`: The intermediate type.
/// - `A`: The target type.
///
/// # Fields
/// - `optic1`: The first optic instance.
/// - `optic2`: The second optic instance.
/// - `error_fn_1`: A function to map `O1`'s getter error to the unified error type `E`.
/// - `error_fn_2`: A function to map `O2`'s getter error to the unified error type `E`.
struct ComposedPrism<P1: Prism<S, I>, P2: Prism<I, A>, E, S, I, A> {
    optic1: P1,
    optic2: P2,
    error_fn_1: fn(P1::GetterError) -> E,
    error_fn_2: fn(P2::GetterError) -> E,
    _phantom: PhantomData<(S, I, A, E)>,
}

impl<P1, P2, E, S, I, A> ComposedPrism<P1, P2, E, S, I, A>
where
    P1: Prism<S, I>,
    P2: Prism<I, A>,
{
    fn new(
        optic1: P1,
        optic2: P2,
        error_fn_1: fn(P1::GetterError) -> E,
        error_fn_2: fn(P2::GetterError) -> E,
    ) -> Self {
        ComposedPrism {
            optic1,
            optic2,
            error_fn_1,
            error_fn_2,
            _phantom: PhantomData,
        }
    }
}

impl<P1, P2, E, S, I, A> HasGetter<S, A> for ComposedPrism<P1, P2, E, S, I, A>
where
    P1: Prism<S, I>,
    P2: Prism<I, A>,
{
    type GetterError = E;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self.optic1.try_get(source).map_err(self.error_fn_1)?;
        self.optic2.try_get(&i).map_err(self.error_fn_2)
    }
}

impl<P1, P2, E, S, I, A> HasSetter<S, A> for ComposedPrism<P1, P2, E, S, I, A>
where
    P1: Prism<S, I>,
    P2: Prism<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut i) = self.optic1.try_get(source).map_err(self.error_fn_1) {
            self.optic2.set(&mut i, value);
            self.optic1.set(source, i);
        }
    }
}

/// Creates a `Prism<S,A>` combined from two optics <S, I>, <I, A> applied one after another.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `composable_with_XXX` methods, where the two optics can be of any
/// valid optic type that results in a `Prism`.
///
/// # Type Parameters
/// - `S`: The source type of the first optic
/// - `A`: The target type of the second optic
/// - `I`: The intermediate type: the target type of the first optic and the source type of the second optic
/// - `E`: The error type for the resulting optic
///
/// # Arguments
/// - `p1`: The first optic of type `Prism<S, I>`
/// - `p2`: The second optic of type `Prism<I, A>`
/// - `error_fn_1`: A function that maps the error type of the first optic to a resulting error type `E`
/// - `error_fn_2`: A function that maps the error type of the second optic to a resulting error type `E`
///
/// This struct **should not** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `compose_with_XXX` methods on each optic impl.
/// The `ComposedPrism` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Prism`] — the optic type that `ComposedPrism` is based on
#[must_use]
pub fn new<S, A, I, E, P1: Prism<S, I>, P2: Prism<I, A>>(
    p1: P1,
    p2: P2,
    error_fn_1: fn(P1::GetterError) -> E,
    error_fn_2: fn(P2::GetterError) -> E,
) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>> {
    ComposedPrism::new(p1, p2, error_fn_1, error_fn_2).into()
}
