//TODO: Consider returning a bool here, or adding a PartialSetter trait
/// A base trait for optics that provides a setter operation.
///
/// This trait defines the ability to set a value of type `A` into a mutable source of type `S`.
/// It serves as a foundational trait for constructing more complex optics like lenses and prisms.
///
/// # Implementors
///
/// Types that implement `HasSetter` can be used to define optics that allow for
/// setting values into a source.
///
///   - [`Setter`] — a concrete optic that allows only set operations.
///   - [`Prism`] — optic that allows for fallible retrieval of values.
///   - [`Lens`] — a total optic that allows for setting values.
///   - [`FallibleIso`] — reversible optic that can allows for fallible conversion of values in both directions.///
///   - [`Iso`] — a reversible optic that allows for setting values in both directions.
pub trait HasSetter<S, A> {
    /// Sets a value of type `A` the optic focuses on in a mutable source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `source`: A mutable reference to the source of type `S` into which the value is to be set.
    /// - `value`: The value of type `A` to be set into the source.
    fn set(&self, source: &mut S, value: A);
}
