//TODO: Consider returning a bool here, or adding a PartialSetter trait
pub trait HasSetter<S, A> {
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
