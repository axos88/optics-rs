/// A base trait for optics that provides a partial getter operation.
///
/// This trait defines the ability to retrieve a value of type `A` from a source of type `S`,
/// potentially failing with an error of type `GetterError`. It serves as a foundational trait for
/// constructing more complex optics like lenses and prisms.
///
/// # Associated Types
///
/// - `GetterError`: The type of the error that may occur during retrieval. This will propagete
///   as the error type of retrieval of concrete optics that implement this trait.
///
/// # Notes
/// - Currently, you will likely need to Clone or Copy the result in order to extract it from the source.
///
/// # Implementors
///
/// Types that implement `HasPartialGetter` can be used to define optics that allow for
/// partial retrieval of values from a source, where the retrieval may fail.
///
///   - [`PartialGetter`] — optic that allows only fallible read operations
///   - [`Getter`] — optic that allows only infallible read operations
///   - [`Prism`] — optic that allows for fallible retrieval of values.
///   - [`Lens`] — optic that allows for infallible retrieval of values.
///   - [`FallibleIso`] — reversible optic that can allows for fallible conversion of values in both directions.
///   - [`Iso`] — reversible optic that never fails.
///
pub trait HasGetter<S, A> {
    /// The type of error that may occur during retrieval. Use `Infallible` for infallible optics.
    type GetterError;

    /// Attempts to retrieve a value of type `A` from a source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `source`: A reference to the source of type `S` from which the value is to be retrieved.
    ///
    /// # Errors
    ///
    /// It returns an error specified by the implementing optic if the focus fails.
    ///
    /// # Returns
    ///
    /// Returns a `Result<A, Self::GetterError>`, of the value the optic focuses on.
    fn try_get(&self, source: &S) -> Result<A, Self::GetterError>;
}
