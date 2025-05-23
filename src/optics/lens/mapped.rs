use crate::HasGetter;
use crate::HasSetter;
use crate::optics::lens::Lens;
use crate::optics::lens::wrapper::LensImpl;
use core::convert::Infallible;
use core::marker::PhantomData;

struct MappedLens<S, A, GET = fn(&S) -> A, SET = fn(&mut S, A)>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    get_fn: GET,
    set_fn: SET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, GET, SET> MappedLens<S, A, GET, SET>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    pub(crate) fn new(get_fn: GET, set_fn: SET) -> Self {
        MappedLens {
            get_fn,
            set_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, GET, SET> HasGetter<S, A> for MappedLens<S, A, GET, SET>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        Ok((self.get_fn)(source))
    }
}

impl<S, A, GET, SET> HasSetter<S, A> for MappedLens<S, A, GET, SET>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    fn set(&self, source: &mut S, value: A) {
        (self.set_fn)(source, value);
    }
}

/// Creates a new `Lens` with the provided getter and setter function.
///
/// # Type Parameters
/// - `S`: The source type of the optic
/// - `A`: The target type of the optic
/// # Arguments
///
/// - `get_fn` — A function that retrieves the focus value `A` from the source `S`.
/// - `set_fn` — A function that sets the focused value `A` from the source `S`.
///
/// # Returns
///
/// A new `LensImpl` instance that can be used as a `Lens<S, A>`.
///
/// # Examples
///
/// ```
/// use optics::{mapped_lens, HasSetter, HasTotalGetter};
///
/// #[derive(Debug, PartialEq)]
/// struct Point { x: u32, y: u32 };
/// let x_lens = mapped_lens(
///     |s: &Point| s.x,
///     |s,v| s.x = v
/// );
///
/// let mut p = Point { x: 10, y: 20 };
///
/// assert_eq!(x_lens.get(&p), 10);
/// x_lens.set(&mut p, 42);
/// assert_eq!(x_lens.get(&p), 42);
/// ```
#[must_use]
pub fn new<S, A, GET, SET>(get_fn: GET, set_fn: SET) -> LensImpl<S, A, impl Lens<S, A>>
where
    GET: Fn(&S) -> A,
    SET: Fn(&mut S, A),
{
    MappedLens::new(get_fn, set_fn).into()
}
