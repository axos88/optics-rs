/// A base trait for optics that provides a partial reversible operation.
///
/// This trait defines the ability to reverse a value of type `A` back into a source of type `S`,
/// potentially failing with an error of type `ReverseError`. It serves as a foundational trait for
/// constructing more complex optics like reversible prisms and fallible isomorphisms.
///
/// # Associated Types
///
/// - `ReverseError`: The type of the error that may occur during the reverse operation. This will propagete
/// as the error type of reverse retrieval of concrete optics that implement this trait.
///
/// # Notes
///
/// - Currently, you will likely need to clone or copy the value in order to reverse it into the source.
/// - Logically a `PartialReversible<S, A>` implies `PartialGetter<A, S>`, but I have not yet found a way
/// around the compiler trait cohesion limitations
/// - One way could be to remove `PartialReversible` entirely, and use `PartialGetter<A, S>` instead of
/// `PartialReversible<S, A>`, but that comes with its own set of ergonomics issues, like how to
/// disambuguate between the two `try_get` operations without too much boilerplate.
///
/// # Implementors
///
/// Types that implement `HasPartialReversible` can be used to define optics that allow for
/// partial reversal of values back into a source, where the reversal may fail.
///
///   - [`FallibleIso`] — a reversible optic that can fail in both directions.
///
pub trait HasPartialReversible<S, A> {
    /// The type of error that may occur during the reverse operation.
    type ReverseError;

    /// Attempts to reverse a value of type `A` back into a source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `value`: A reference to the value of type `A` to be reversed into the source.
    ///
    /// # Returns
    ///
    /// Returns a `Result<S, Self::ReverseError>`, of the value the optic focuses on.
    fn try_reverse_get(&self, value: &A) -> Result<S, Self::ReverseError>;
}
