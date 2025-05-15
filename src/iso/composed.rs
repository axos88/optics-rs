use crate::iso::Iso;
use crate::{FallibleIso, IsoImpl, Lens, Optic, Prism};
use core::convert::Infallible;
use core::marker::PhantomData;

/// A composed `Iso` type, combining two optics into a single `Iso`.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type where the result is a `Iso`.
///
/// A `Composed` not only combines two optics into a single lens, but it also inherently
/// acts as a `Prism` and `Optic`. This behavior arises from the fact that a `Iso` is itself a
/// more specific form of an optic, and prism and thus any `Iso` composition will also be usable as
/// a `Prism` and an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedIso` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Iso`] — the core optic type that the `ComposedIso` is based on
/// - [`Prism`] — the optic type that `ComposedIso` also acts as
/// - [`Optic`] — the base trait that all optic types implement
pub struct ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    optic1: O1,
    optic2: O2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<O1, O2, S, I, A> ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    pub(crate) fn new(optic1: O1, optic2: O2) -> Self where {
        ComposedIso {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<O1, O2, S, I, A> Optic<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    type Error = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::Error> {
        self.optic2.try_get(&self.optic1.try_get(source)?)
    }

    fn set(&self, source: &mut S, value: A) {
        let Ok(mut inter) = self.optic1.try_get(source);
        self.optic2.set(&mut inter, value);
        self.optic1.set(source, inter);
    }
}

impl<O1, O2, S, I, A> Prism<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    fn preview(&self, source: &S) -> Option<A> {
        Some(self.get(source))
    }
}

impl<O1, O2, S, I, A> Lens<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    fn get(&self, source: &S) -> A {
        let Ok(i) = self.optic1.try_get(source);
        let Ok(a) = self.optic2.try_get(&i);
        a
    }
}

impl<O1, O2, S, I, A> FallibleIso<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    fn try_reverse_get(&self, value: &A) -> Result<S, Self::Error> {
        let i = self.optic2.reverse_get(value);
        Ok(self.optic1.reverse_get(&i))
    }
}

impl<O1, O2, S, I, A> Iso<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    fn reverse_get(&self, value: &A) -> S {
        let i = self.optic2.reverse_get(value);
        self.optic1.reverse_get(&i)
    }
}

pub fn new<S, A, I, E, F1: Iso<S, I>, F2: Iso<I, A>>(
    f1: F1,
    f2: F2,
) -> IsoImpl<S, A, ComposedIso<F1, F2, S, I, A>>
where
    F1::Error: Into<E>,
    F2::Error: Into<E>,
{
    IsoImpl::new(ComposedIso::new(f1, f2))
}
