/// A bidirectional, fallible isomorphism between two types `S` and `A`.
///
/// A `FallibleIso` is an optic that provides a potentially lossy, reversible mapping between a
/// source type `S` and a focus type `A`, where **both** the forward (`S → A`) and reverse
/// (`A → S`) transformations can fail independently.
///
/// This makes it suitable for conversions where neither direction is guaranteed to succeed in
/// all cases. Examples include parsing, type coercion, or partial decoding tasks where values
/// may not always be representable in the other form.
///
/// # Supertraits
/// - [`Optic<S, A>`] — provides the primary optic interface for fallible `get` and `set` operations.
/// - [`Prism<S, A>`] — allows using this `FallibleIso` as a `Prism`.
///
/// # Error Semantics
/// The associated `Error` type on the `Optic` supertrait defines the possible error value for
/// both the `try_get` and `try_reverse_get` operations.
///
/// # See Also
/// - [`Iso`] — for total, infallible isomorphisms.
/// - [`Prism`] — for partial optics where only one direction may be partial.
/// - [`Optic`] — the base trait for all optics.
pub trait PartialReversible<S, A> {
    type ReverseError;
    /// Attempts to perform the reverse transformation from the focus type `A` back to the source type `S`.
    ///
    /// Since this is a *fallible* isomorphism, the operation may fail if the provided `A` value
    /// cannot be converted back into a valid `S`. The error type is defined by the `Error`
    /// associated type of the [`Optic`] supertrait.
    ///
    /// # Arguments
    /// * `source` — A reference to the focus type value `A`.
    ///
    /// # Returns
    /// `Ok(S)` if the reverse transformation succeeds,
    ///
    /// # Errors
    /// Returns `Err(Self::Error)` if the transformation fails.
    ///
    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError>;
}
