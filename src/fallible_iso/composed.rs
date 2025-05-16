use crate::fallible_iso::{FallibleIso, FallibleIsoImpl};
use crate::{NoFocus, Optic, Prism};
use core::marker::PhantomData;

/// A composed `FallibleIso` type, combining two optics into a single `FallibleIso`.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type where the result is a `FallibleIso`.
///
/// A `ComposedFallible` not only combines two optics into a single lens, but it also inherently
/// acts as a `Prism` and `Optic`. This behavior arises from the fact that a `FallibleIso` is itself a
/// more specific form of an optic, and prism and thus any `FallibleIso` composition will also be usable as
/// a `Prism` and an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedFallibleIso` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`FallibleIso`] — the core optic type that the `ComposedFallibleIso` is based on
/// - [`Prism`] — the optic type that `ComposedFallibleIso` also acts as
/// - [`Optic`] — the base trait that all optic types implement
pub struct ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
{
    optic1: O1,
    optic2: O2,
    _phantom: PhantomData<(S, I, A, E)>,
}

impl<O1, O2, S, I, A, E> ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
{
    pub(crate) fn new(optic1: O1, optic2: O2) -> Self where {
        ComposedFallibleIso {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<O1, O2, E, S, I, A> Optic<S, A> for ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
    O2::Error: Into<E>,
    O1::Error: Into<E>,
{
    type Error = E;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.optic2
            .try_get(&self.optic1.try_get(source).map_err(Into::into)?)
            .map_err(Into::into)
    }

    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut inter) = self.optic1.try_get(source) {
            self.optic2.set(&mut inter, value);
            self.optic1.set(source, inter);
        }
    }
}

impl<O1, O2, E, S, I, A> FallibleIso<S, A> for ComposedFallibleIso<O1, O2, E, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
    O1::Error: Into<E>,
    O2::Error: Into<E>,
{
    fn try_reverse_get(&self, source: &A) -> Result<S, Self::Error> {
        let i = self.optic2.try_reverse_get(source).map_err(Into::into)?;
        self.optic1.try_reverse_get(&i).map_err(Into::into)
    }
}

pub fn new<S, A, I, E, F1: FallibleIso<S, I>, F2: FallibleIso<I, A>>(
    f1: F1,
    f2: F2,
) -> FallibleIsoImpl<S, A, ComposedFallibleIso<F1, F2, E, S, I, A>>
where
    F1::Error: Into<E>,
    F2::Error: Into<E>,
{
    FallibleIsoImpl::new(ComposedFallibleIso::new(f1, f2))
}
