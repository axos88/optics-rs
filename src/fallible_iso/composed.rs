use crate::HasPartialReversible;
use crate::fallible_iso::FallibleIso;
use crate::fallible_iso::wrapper::FallibleIsoImpl;
use crate::{HasPartialGetter, HasSetter};
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
struct ComposedFallibleIso<O1, O2, GE, RE, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
{
    optic1: O1,
    optic2: O2,
    getter_error_fn_1: fn(O1::GetterError) -> GE,
    getter_error_fn_2: fn(O2::GetterError) -> GE,
    reverse_error_fn_1: fn(O1::ReverseError) -> RE,
    reverse_error_fn_2: fn(O2::ReverseError) -> RE,
    _phantom: PhantomData<(S, I, A, GE, RE)>,
}

impl<O1, O2, S, I, A, GE, RE> ComposedFallibleIso<O1, O2, GE, RE, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
{
    pub(crate) fn new(
        optic1: O1,
        optic2: O2,
        getter_error_fn_1: fn(O1::GetterError) -> GE,
        getter_error_fn_2: fn(O2::GetterError) -> GE,
        reverse_error_fn_1: fn(O1::ReverseError) -> RE,
        reverse_error_fn_2: fn(O2::ReverseError) -> RE,
    ) -> Self where {
        ComposedFallibleIso {
            optic1,
            optic2,
            getter_error_fn_1,
            getter_error_fn_2,
            reverse_error_fn_1,
            reverse_error_fn_2,
            _phantom: PhantomData,
        }
    }
}

impl<O1, O2, GE, RE, S, I, A> HasPartialGetter<S, A>
    for ComposedFallibleIso<O1, O2, GE, RE, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
{
    type GetterError = GE;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self
            .optic1
            .try_get(source)
            .map_err(self.getter_error_fn_1)?;
        self.optic2.try_get(&i).map_err(self.getter_error_fn_2)
    }
}

impl<O1, O2, GE, RE, S, I, A> HasPartialReversible<S, A>
    for ComposedFallibleIso<O1, O2, GE, RE, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
{
    type ReverseError = RE;

    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError> {
        let i = self
            .optic2
            .try_reverse_get(value)
            .map_err(self.reverse_error_fn_2)?;
        self.optic1
            .try_reverse_get(&i)
            .map_err(self.reverse_error_fn_1)
    }
}

impl<O1, O2, GE, RE, S, I, A> HasSetter<S, A> for ComposedFallibleIso<O1, O2, GE, RE, S, I, A>
where
    O1: FallibleIso<S, I>,
    O2: FallibleIso<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        if let Ok(mut i) = self.optic1.try_get(source).map_err(self.getter_error_fn_1) {
            self.optic2.set(&mut i, value);
            self.optic1.set(source, i);
        }
    }
}

#[must_use] pub fn new<S, A, I, GE, RE, F1: FallibleIso<S, I>, F2: FallibleIso<I, A>>(
    f1: F1,
    f2: F2,
    getter_error_fn_1: fn(F1::GetterError) -> GE,
    getter_error_fn_2: fn(F2::GetterError) -> GE,
    reverse_error_fn_1: fn(F1::ReverseError) -> RE,
    reverse_error_fn_2: fn(F2::ReverseError) -> RE,
) -> FallibleIsoImpl<S, A, impl FallibleIso<S, A, GetterError = GE, ReverseError = RE>>
where
{
    FallibleIsoImpl::new(ComposedFallibleIso::new(
        f1,
        f2,
        getter_error_fn_1,
        getter_error_fn_2,
        reverse_error_fn_1,
        reverse_error_fn_2,
    ))
}
