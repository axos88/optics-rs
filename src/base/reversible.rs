use core::convert::Infallible;
use crate::{HasPartialReversible};

/// A base trait for optics that provides a total reversible operation.
///
/// This trait defines the ability to reverse a value of type `A` back into a source of type `S`,
/// without the possibility of failure. It serves as a foundational trait for constructing
/// more complex optics like lenses and isomorphisms.
///
/// # Notes
///
/// - Currently, you will likely need to clone or copy the value in order to reverse it into the source.
/// - Logically a `Reversible<S, A>` should imply `Getter<A, S>`, but I have not yet found a way
/// around the compiler trait cohesion limitations
/// - One way could be to remove `Reversible` entirely, and use `Getter<A, S>` instead of
/// `Reversible<S, A>`, but that comes with its own set of ergonomics issues, like how to
/// disambuguate between the two `get` operations without too much boilerplate.
///
/// # Implementors
///
/// Types that implement `HasReversible` can be used to define optics that allow for
/// total reversal of values back into a source, where the reversal is guaranteed to succeed.
///
///   - [`Iso`] — a reversible optic that allows for infallible retrieval and reversal of values.
///   - [`Lens`] — a total optic that allows for infallible retrieval and reversal of values.
///
/// # Notes
///
/// Currently, you will likely need to clone or copy the value in order to reverse it into the source.
pub trait HasReversible<S, A> {
    /// Reverses a value of type `A` back into a source of type `S`.
    ///
    /// # Parameters
    ///
    /// - `value`: A reference to the value of type `A` to be reversed into the source.
    ///
    /// # Returns
    ///
    /// Returns the source of type `S`.
    fn reverse_get(&self, value: &A) -> S;
}

impl<S, A, T> HasReversible<S, A> for T where T: HasPartialReversible<S, A, ReverseError = Infallible>
{
    fn reverse_get(&self, value: &A) -> S {
        match self.try_reverse_get(value) {
            Ok(s) => s
        }
    }
}