mod composed;
mod mapped;
mod wrapper;

use crate::{mapped_partial_getter, HasSetter};

pub use composed::new as composed_setter;
pub use mapped::new as mapped_setter;
pub use wrapper::SetterImpl;

/// A `Setter` is an optic that can change its focused value, providing
/// only a write operation
///
/// It provides:
/// - `set` to set the focused value inside a larger type
///
/// This is useful when providing a way to save a particular part of a larger type without giving
/// any access to its contents.
///
/// Type Arguments
///   - `S`: The data type the optic operates on
///   - `A`: The data type the optic focuses on
///
/// # Note
///
/// This is a marker trait that is blanket implemented for all structs that satisfy the requirements.
///
/// # See Also
/// - [`Prism`] — an optic that focuses on a potentially missing value in a product type (ex. optional struct field) or a sum type vairant
/// - [`Lens`] — an optic that focuses on an always-present value in a product type (e.g., a required struct field)
/// - [`FallibleIso`] — a variant of `Iso` where the mapping might fail, returning an error
/// - [`Iso`] — an isomorphism optic representing a reversible bijective conversion between two types
pub trait Setter<S, A>: HasSetter<S, A> {}

impl<S, A, SETTER: HasSetter<S, A>> Setter<S, A> for SETTER {}

/// Creates a `Setter` that focuses on the entire input.
///
/// It can be useful in cases where you want to restrict a part of the codebase to be unable to read
/// a value while still allowing it to write to it.
///
/// # Type Parameters
///
/// - `S`: The type of the input and output value.
///
/// # Returns
///
/// A `SetterImpl` instance that implements `Setter<S, S>`
///
/// # Example
///
///```rust
/// use optics::{identity_setter, HasSetter, Setter, SetterImpl};
///
/// struct Foo(u32);
///
/// impl Default for Foo {
///  fn default() -> Self {
///         Foo(42)
///   }
/// }
///
/// fn set_to_default<T: Default>(val: &mut T, setter: SetterImpl<T, T, impl Setter<T, T>>) {
///  // We have no way to access what val currently is, we don't even know it's type!
///  setter.set(val, T::default());
/// }
/// 
/// let mut s = Foo(142);
///
/// set_to_default(&mut s, identity_setter());
///
/// assert_eq!(s.0, 42);
/// ```
///
/// # See Also
///
/// - [`mapped_setter`] for constructing custom `Setter`s from arbitrary mapping functions.
#[must_use]
pub fn identity_setter<S>() -> SetterImpl<S, S, impl Setter<S, S>> {
    mapped_setter(|s, v| *s = v)
}
