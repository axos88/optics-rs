use crate::{
    HasGetter, HasPartialGetter, HasPartialReversible, HasReversible, HasSetter,
};
use core::convert::Infallible;

mod composed;
mod mapped;
mod wrapper;

pub use composed::new as composed_iso;
pub use mapped::new as mapped_iso;
pub use wrapper::IsoImpl;

/// An isomorphism between two types `S` and `A`.
///
/// An `Iso` is a bidirectional optic that provides a one-to-one, lossless, and reversible mapping
/// between a source type `S` and a focus type `A`. Unlike general `Prism` or `Lens` optics, an
/// `Iso` guarantees that for every `S` there exists exactly one corresponding `A`, and vice versa.
///
/// Because it is total and invertible, an `Iso` is both a `Lens` and a `Prism`, as well as a
/// `FallibleIso` with an infallible error type (`Infallible`). This means it participates fully
/// in the optic hierarchy, providing total reads, total writes, and reversible transformations.
///
/// # Supertraits
/// - [`Optic<S, A, Error = Infallible>`] — ensures that operations on the `Iso` cannot fail.
/// - [`FallibleIso<S, A>`] — provides the fallible isomorphism API, but with `Infallible` error.
/// - [`Prism<S, A>`] — allows using this `Iso` as a `Prism`.
/// - [`Lens<S, A>`] — allows using this `Iso` as a `Lens`.
///
/// # See Also
/// - [`Lens`] — for total, read/write optics.
/// - [`Prism`] — for partial optics.
/// - [`FallibleIso`] — for reversible optics that can fail.
/// - [`Optic`] — the base trait for all optics.
pub trait Iso<S, A>:
    HasPartialGetter<S, A, GetterError = Infallible>
    + HasSetter<S, A>
    + HasPartialReversible<S, A, ReverseError = Infallible>
{
}

impl<
    S,
    A,
    ISO: HasPartialGetter<S, A, GetterError = Infallible>
        + HasSetter<S, A>
        + HasPartialReversible<S, A, ReverseError = Infallible>,
> Iso<S, A> for ISO
{
}

#[must_use] pub fn identity_iso<S: Clone>() -> IsoImpl<S, S, impl Iso<S, S>> {
    mapped_iso(|x: &S| x.clone(), |x: &S| x.clone())
}
