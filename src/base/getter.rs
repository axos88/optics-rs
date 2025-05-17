/// A base trait for optics that provides a total getter operation.
///
/// This trait defines the ability to retrieve a value of type `A` from a source of type `S`,
/// without the possibility of failure. It serves as a foundational trait for constructing
/// more complex optics like lenses and prisms.
///
/// # Notes
/// Currently, you will likely need to clone or copy the result in order to extract it from the source.
///
/// # Implementors
///
/// Types that implement `HasGetter` can be used to define optics that allow for
/// total retrieval of values from a source, where the retrieval is guaranteed to succeed.
///
///   - [`Getter`] — optic that allows only infallible read operations.
///   - [`Lens`] — optic that allows for infallible retrieval of values.
///   - [`Iso`] — a reversible optic that allows for infallible conversion of values in both directions.
pub trait HasGetter<S, A> {
    /// Retrieves a value of type `A` from a source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `source`: A reference to the source of type `S` from which the value is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns the value of type `A` that the optic focuses on.
    fn get(&self, source: &S) -> A;
}
