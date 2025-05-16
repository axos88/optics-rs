pub trait Getter<S, A> {
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
    fn get(&self, source: &S) -> A;
}
