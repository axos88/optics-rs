use crate::optics::partial_getter::wrapper::PartialGetterImpl;
use crate::{HasGetter, PartialGetter};
use core::marker::PhantomData;

/// A concrete implementation of the [`PartialGetter`] trait.
///
/// This struct allows you to create a `PartialGetter` by providing custom getter and setter functions.
/// It is the primary way to create a `PartialGetter` manually, and is flexible enough to support any
/// getter and setter functions that match the required signatures.
///
/// Typically, you will only need to specify the source type `S` and the focus type `A`.
/// The getter and setter functions will be inferred or explicitly provided.
///
/// If you prefer to use capturing closures, you can specify the trailing type parameters as `_`
/// in the construction of `PartialGetterImpl` to allow the compiler to infer them for you. This is especially
/// useful when you need to use closures with captured environment variables.
///
/// # Construction
///
/// The usual way to construct a `PartialGetterImpl` is to use `PartialGetterImpl::<S, A>::new()`, which
/// will use non-capturing closures by default. Alternatively, you can specify the
/// getter and setter type parameters as `PartialGetterImpl::<S, A,_, _>::new()` for more complex use cases,
/// such as captruring closures.
///
/// # See Also
///
/// - [`Lens`] — a more restrictive optic type for focus values
/// - [`Optic`] — base trait that all optics implement
struct MappedPartialGetter<S, A, E, GET = fn(&S) -> Result<A, E>>
where
    GET: Fn(&S) -> Result<A, E>,
{
    get_fn: GET,
    phantom: PhantomData<(S, A)>,
}

impl<S, A, E, GET> MappedPartialGetter<S, A, E, GET>
where
    GET: Fn(&S) -> Result<A, E>,
{
    fn new(get_fn: GET) -> Self {
        MappedPartialGetter {
            get_fn,
            phantom: PhantomData,
        }
    }
}

impl<S, A, E, GET> HasGetter<S, A> for MappedPartialGetter<S, A, E, GET>
where
    GET: Fn(&S) -> Result<A, E>,
{
    type GetterError = E;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        (self.get_fn)(source)
    }
}

/// Creates a new `PartialGetter` with the provided getter function.
///
/// # Type Parameters
/// - `S`: The source type of the optic
/// - `A`: The target type of the optic
/// - `E`: The error type returned when the focus fails

/// # Arguments
///
/// - `get_fn` — A function that faillibly retrieves the focus value `A` from the source `S`.
///
/// # Returns
///
/// A new `PartialGetterImpl` instance that can be used as a `PartialGetter<S, A>`.
///
/// # Examples
///
/// ```
/// use optics::{mapped_partial_getter, HasGetter};
///
/// enum IpAddress { Ipv4(String), Ipv6(String) }
/// let ipv4_partial_getter = mapped_partial_getter(
///     |s: &IpAddress| if let IpAddress::Ipv4(ip) = s { Ok(ip.clone()) } else { Err(()) }
/// );
///
/// let addr = IpAddress::Ipv4("8.8.4.4".to_string());
///
/// assert_eq!(ipv4_partial_getter.try_get(&addr), Ok("8.8.4.4".to_string()));
/// ```
#[must_use]
pub fn new<S, A, E, GET>(
    get_fn: GET,
) -> PartialGetterImpl<S, A, impl PartialGetter<S, A, GetterError = E>>
where
    GET: Fn(&S) -> Result<A, E>,
{
    MappedPartialGetter::new(get_fn).into()
}
