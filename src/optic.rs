/// The base trait for all optics in this library.
///
/// An `Optic` is a general abstraction representing a way to access and update a value (`A`)
/// inside a larger data structure (`S`). The optic attempts to extract the focus value via
/// `try_get` and can replace it via `set`.
///
/// This trait serves as the common foundation for more specialized optic types:
///
/// - [`crate::Lens`] — always succeeds in retrieving a focus value in product types (e.g., struct fields)
/// - [`crate::Prism`] — may fail if the focus is absent, typically for sum types (e.g., enum variants)
/// - [`crate::Iso`] — a reversible bijective transformation that always succeeds
/// - [`crate::FallibleIso`] — a reversible transformation where the mapping can fail
///
/// # See Also
///
/// - [`crate::Prism`] — optional focus in sum types, may fail to extract focus
/// - [`crate::Lens`] — always-present focus in product types
/// - [`crate::FallibleIso`] — reversible transformations with fallible forward mapping
/// - [`crate::Iso`] — reversible one-to-one transformations
pub trait Optic<S, A> {
    /// The error type returned when focus extraction via `try_get` fails.
    ///
    /// - [`crate::Lens`] typically uses [`core::convert::Infallible`]
    /// - [`crate::Prism`] typically uses [`crate::NoFocus`]
    /// - [`crate::Iso`] uses `Infallible`
    /// - [`crate::FallibleIso`] uses a custom error type for forward mapping failures
    type Error;

    /// Attempts to extract the focus value `A` from a source `S`.
    ///
    /// # Arguments
    ///
    /// - `source` — A reference to the source structure containing the focus.
    ///
    /// # Returns
    ///
    /// - `Ok(A)` if focus extraction is successful.
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` if the focus is absent or extraction fails.
    ///
    ///
    /// # Behavior
    ///
    /// This operation is pure and should not have side effects.
    fn try_get(&self, source: &S) -> Result<A, Self::Error>;

    /// Replaces the focus value inside the source structure with a new value.
    ///
    /// # Arguments
    ///
    /// - `source` — A mutable reference to the source structure containing the focus.
    /// - `value` — The new value to assign to the focus.
    ///
    /// # Behavior
    ///
    /// - For [`crate::Lens`] and [`crate::Iso`], this operation should always succeed in setting the value.
    /// - For [`crate::Prism`] and [`crate::FallibleIso`], this may be a no-op or if no focus is present.
    ///
    /// This method consumes `value` by value and updates the source in-place.
    fn set(&self, source: &mut S, value: A);
}
