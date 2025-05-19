use crate::iso::Iso;
use crate::iso::wrapper::IsoImpl;
use crate::{HasGetter, HasPartialGetter, HasPartialReversible, HasReversible, HasSetter};
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
struct ComposedIso<O1, O2, S, I, A>
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

impl<O1, O2, S, I, A> HasPartialGetter<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok(self.get(source))
    }
}

impl<O1, O2, S, I, A> HasGetter<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    fn get(&self, source: &S) -> A {
        let i = self.optic1.get(source);
        self.optic2.get(&i)
    }
}

impl<O1, O2, S, I, A> HasSetter<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        let mut i = self.optic1.get(source);
        self.optic2.set(&mut i, value);
        self.optic1.set(source, i);
    }
}

impl<O1, O2, S, I, A> HasPartialReversible<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    type ReverseError = Infallible;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        Ok(self.reverse_get(value))
    }
}

impl<O1, O2, S, I, A> HasReversible<S, A> for ComposedIso<O1, O2, S, I, A>
where
    O1: Iso<S, I>,
    O2: Iso<I, A>,
{
    fn reverse_get(&self, value: &A) -> S {
        let i = self.optic2.reverse_get(value);
        self.optic1.reverse_get(&i)
    }
}

#[must_use] pub fn new<S, A, I, F1: Iso<S, I>, F2: Iso<I, A>>(f1: F1, f2: F2) -> IsoImpl<S, A, impl Iso<S, A>>
where
{
    ComposedIso::new(f1, f2).into()
}
