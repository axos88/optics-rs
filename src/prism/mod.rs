use crate::HasPartialGetter;
use crate::HasSetter;
use core::convert::Infallible;

mod composed;
mod mapped;
mod wrapper;

pub use composed::new as composed_prism;
pub use mapped::new as mapped_prism;
pub use wrapper::PrismImpl;

/// An optic that focuses on a part of a sum type, allowing for partial access and construction.
///
/// A `Prism` is an optic used to work with sum types (also known as coproducts), such as Rust's `enum`s. It provides the ability to:
///
/// - **Attempt to extract** a focused value of type `A` from a source of type `S`, potentially failing if the value is not present.
/// - **Construct** a value of type `S` from a focused value of type `A`.
///
/// This is particularly useful for working with types like `Option`, `Result`, or custom enums, where a value may or may not be present.
///
/// # Trait Definition
///
/// The `Prism` trait extends the [`HasPartialGetter`] and [`HasSetter`] traits:
///
/// - [`HasPartialGetter<S, A>`]: Provides the `try_get` method to attempt extraction of a value of type `A` from `S`.
/// - [`HasSetter<S, A>`]: Provides the `set` method to construct a value of type `S` from `A`.
///
/// Together, these traits allow for partial access and construction, embodying the essence of a `Prism`.
///
/// # Usage
///
/// Prisms are ideal for scenarios where you need to work with a specific variant of a sum type. For example, extracting the `Some` value from an `Option`, or the `Ok` value from a `Result`.
///
/// # Notes
///
/// - The setter should always construct a value, even if the getter would otherwise fail. Calling
/// set on an Ok prism should always result in an Ok value, even if the previous focus was on an Err.
///
/// - Implementing this trait manually is generally discouraged unless you are working on a new prism implementation.
/// Instead, use the provided implementations or constructors within the crate to ensure consistency and correctness.
///
/// # See Also
///
/// - [`HasPartialGetter`]: A trait for types that can partially extract a value.
/// - [`Setter`]: A trait for types that can set a value.
///
pub trait Prism<S, A>: HasPartialGetter<S, A> + HasSetter<S, A> {}

impl<S, A, P: HasPartialGetter<S, A> + HasSetter<S, A>> Prism<S, A> for P {}

/// Constructs an identity `PrismImpl` that focuses on the entire value of type `S`.
///
/// This function creates a `PrismImpl` that acts as a no-op or identity prism, meaning it
/// focuses on the entire structure without any transformation or filtering.
///
/// # Type Parameters
/// - `S`: The type of the value to focus on. This type must implement the `Clone` trait.
///
/// # Returns
/// a new prism that focuses on the entire value of type `S`.
pub fn identity_prism<S: Clone>() -> PrismImpl<S, S, impl Prism<S, S, GetterError = Infallible>> {
    mapped_prism(|s: &S| Ok::<_, Infallible>(s.clone()), |_, _| ())
}
