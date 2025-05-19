use crate::iso::Iso;
use crate::iso::wrapper::IsoImpl;
use crate::{HasGetter, HasReverseGet, HasSetter, HasTotalGetter, HasTotalReverseGet};
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

#[must_use]
pub fn new<S, A, I, ISO1: Iso<S, I>, ISO2: Iso<I, A>>(
    f1: ISO1,
    f2: ISO2,
) -> IsoImpl<S, A, impl Iso<S, A>>
where
{
    ComposedIso::new(f1, f2).into()
}
