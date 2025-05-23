use crate::HasSetter;
use crate::{Setter, SetterImpl};
use core::marker::PhantomData;

struct MappedSetter<S, A, SET = fn(&mut S, A)>
where
    SET: Fn(&mut S, A),
{
    set_fn: SET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, SET> MappedSetter<S, A, SET>
where
    SET: Fn(&mut S, A),
{
    fn new(set_fn: SET) -> Self {
        MappedSetter {
            set_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, SET> HasSetter<S, A> for MappedSetter<S, A, SET>
where
    SET: Fn(&mut S, A),
{
    fn set(&self, source: &mut S, value: A) {
        (self.set_fn)(source, value);
    }
}

/// Creates a new `Setter` with the provided setter function.
///
/// # Type Parameters
/// - `S`: The source type of the optic
/// - `A`: The target type of the optic
/// # Arguments
///
/// - `set_fn` — A function that sets the focus value `A` from the source `S`.
///
/// # Returns
///
/// A new `SetterImpl` instance that can be used as a `Setter<S, A>`.
///
/// # Examples
///
/// ```
/// use optics::{mapped_setter, HasSetter, Setter, SetterImpl};
///
/// struct Point { x: u32, y: u32 };
///
/// let mut s = Point { x: 1, y: 2 };
/// let setter = mapped_setter(|p: &mut Point, v| p.x = v);
///
/// setter.set(&mut s, 42);
///
/// assert_eq!(s.x, 42);
/// ```
#[must_use]
pub fn new<S, A, SET>(set_fn: SET) -> SetterImpl<S, A, impl Setter<S, A>>
where
    SET: Fn(&mut S, A),
{
    MappedSetter::new(set_fn).into()
}
