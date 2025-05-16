pub trait PartialGetter<S, A> {
  /// The error type returned when focus extraction via `try_get` fails.
  ///
  /// - [`crate::Lens`] typically uses [`core::convert::Infallible`]
  /// - [`crate::Prism`] typically uses [`crate::NoFocus`]
  /// - [`crate::Iso`] uses `Infallible`
  /// - [`crate::FallibleIso`] uses a custom error type for forward mapping failures
  type GetterError;

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
  fn try_get(&self, source: &S) -> Result<A, Self::GetterError>;
}