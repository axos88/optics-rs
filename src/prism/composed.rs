use crate::HasPartialGetter;
use crate::HasSetter;
use crate::prism::Prism;
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
pub struct ComposedPrism<O1: Prism<S, I>, O2: Prism<I, A>, E, S, I, A> {
    optic1: O1,
    optic2: O2,
    error_fn_1: fn(O1::GetterError) -> E,
    error_fn_2: fn(O2::GetterError) -> E,
    _phantom: PhantomData<(S, I, A, E)>,
}

impl<O1, O2, E, S, I, A> ComposedPrism<O1, O2, E, S, I, A>
where
    O1: Prism<S, I>,
    O2: Prism<I, A>,
{
    pub(crate) fn new(
        optic1: O1,
        optic2: O2,
        error_fn_1: fn(O1::GetterError) -> E,
        error_fn_2: fn(O2::GetterError) -> E,
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

impl<O1, O2, E, S, I, A> HasPartialGetter<S, A> for ComposedPrism<O1, O2, E, S, I, A>
where
    O1: Prism<S, I>,
    O2: Prism<I, A>,
{
    type GetterError = E;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self.optic1.try_get(source).map_err(self.error_fn_1)?;
        self.optic2.try_get(&i).map_err(self.error_fn_2)
    }
}

impl<O1, O2, E, S, I, A> HasSetter<S, A> for ComposedPrism<O1, O2, E, S, I, A>
where
    O1: Prism<S, I>,
    O2: Prism<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut i) = self.optic1.try_get(source).map_err(self.error_fn_1) {
            self.optic2.set(&mut i, value);
            self.optic1.set(source, i);
        }
    }
}

impl<O1, O2, E, S, I, A> Prism<S, A> for ComposedPrism<O1, O2, E, S, I, A>
where
    O1: Prism<S, I>,
    O2: Prism<I, A>,
{
}

/// Constructs a new `ComposedPrism` by composing two optics, resulting in a `Prism` that focuses
/// from a source type `S` to a target type `A` through an intermediate type `I`.
///
/// This function enables the composition of various combinations of optics, such as:
/// - `Prism` + `Prism`
/// - `Prism` + `Lens`
/// - `FallibleIso` + `Lens`
///
/// The composition handles the potential fallibility of each optic by providing error mapping
/// functions to unify different error types into a single error type `E`.
///
/// # Type Parameters
/// - `S`: The source type.
/// - `A`: The target type.
/// - `I`: The intermediate type.
/// - `E`: The unified error type resulting from composing the getter errors of `L1` and `L2`.
/// - `P1`: The first optic, acting as a `Prism<S, I>`.
/// - `P2`: The second optic, acting as a `Prism<I, A>`.
///
/// # Parameters
/// - `p1`: The first optic instance, focusing from `S` to `I`.
/// - `p2`: The second optic instance, focusing from `I` to `A`.
/// - `error_fn_1`: A function to map `P1`'s getter error to the unified error type `E`.
/// - `error_fn_2`: A function to map `P2`'s getter error to the unified error type `E`.
///
/// # Returns
/// A new `ComposedPrism<P1, P2, E, S, I, A>` instance.
///
pub fn new<S, A, I, E, P1: Prism<S, I>, P2: Prism<I, A>>(
    p1: P1,
    p2: P2,
    error_fn_1: fn(P1::GetterError) -> E,
    error_fn_2: fn(P2::GetterError) -> E,
) -> ComposedPrism<P1, P2, E, S, I, A> {
    ComposedPrism::new(p1, p2, error_fn_1, error_fn_2)
}
