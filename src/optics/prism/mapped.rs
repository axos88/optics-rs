use crate::HasGetter;
use crate::HasSetter;
use crate::optics::prism::Prism;
use crate::optics::prism::wrapper::PrismImpl;
use core::marker::PhantomData;

struct MappedPrism<S, A, E, GET = fn(&S) -> Result<A, E>, SET = fn(&mut S, A)>
where
    GET: Fn(&S) -> Result<A, E>,
    SET: Fn(&mut S, A),
{
    get_fn: GET,
    set_fn: SET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, E, GET, SET> MappedPrism<S, A, E, GET, SET>
where
    GET: Fn(&S) -> Result<A, E>,
    SET: Fn(&mut S, A),
{
    pub(crate) fn new(get_fn: GET, set_fn: SET) -> Self {
        MappedPrism {
            get_fn,
            set_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, E, GET, SET> HasGetter<S, A> for MappedPrism<S, A, E, GET, SET>
where
    GET: Fn(&S) -> Result<A, E>,
    SET: Fn(&mut S, A),
{
    type GetterError = E;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        (self.get_fn)(source)
    }
}

impl<S, A, E, GET, SET> HasSetter<S, A> for MappedPrism<S, A, E, GET, SET>
where
    GET: Fn(&S) -> Result<A, E>,
    SET: Fn(&mut S, A),
{
    fn set(&self, source: &mut S, value: A) {
        (self.set_fn)(source, value);
    }
}

/// Creates a new `Prism` with the provided getter and setter function.
///
/// # Type Parameters
/// - `S`: The source type of the optic
/// - `A`: The target type of the optic
/// - `E`: The error type returned when the focus fails
/// # Arguments
///
/// - `get_fn` — A function that faillibly retrieves the focus value `A` from the source `S`.
/// - `set_fn` — A function that sets the focused value `A` in the source `S`.
///
/// # Examples
///
/// ```
/// use optics::{mapped_prism, HasGetter, HasSetter};
///
/// #[derive(Debug, PartialEq)]
/// enum IpAddress { Ipv4(String), Ipv6(String) }
/// let ipv4_prism = mapped_prism(
///     |s: &IpAddress| if let IpAddress::Ipv4(ip) = s { Ok(ip.clone()) } else { Err(()) },
///     |s,v| *s = IpAddress::Ipv4(v),
/// );
///
/// let mut addr = IpAddress::Ipv4("8.8.4.4".to_string());
///
/// assert_eq!(ipv4_prism.try_get(&addr), Ok("8.8.4.4".to_string()));
/// ipv4_prism.set(&mut addr, "1.1.2.2".to_string());
/// assert_eq!(addr, IpAddress::Ipv4("1.1.2.2".to_string()));
/// ```
#[must_use]
pub fn new<S, A, E, GET, SET>(
    get_fn: GET,
    set_fn: SET,
) -> PrismImpl<S, A, impl Prism<S, A, GetterError = E>>
where
    GET: Fn(&S) -> Result<A, E>,
    SET: Fn(&mut S, A),
{
    MappedPrism::new(get_fn, set_fn).into()
}
