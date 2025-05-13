use crate::fallible_iso::{ComposableFallibleIso, FallibleIso, FallibleIsoImpl};
use crate::iso::{ComposableIso, Iso, IsoImpl};
use crate::lens::Lens;
use crate::lens::{ComposableLens, LensImpl};
use crate::optic::Optic;
use crate::prism::{ComposablePrism, NoFocus, Prism, PrismImpl};
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;

macro_rules! assert_impl {
    ($val:ident : $trait:path) => {{
        fn _assert_impl<T: $trait>(_v: &T) {}
        _assert_impl(&$val);
    }};
}

#[derive(Debug, Clone, PartialEq)]
enum Timespan {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

#[derive(Debug, Clone, PartialEq)]
struct Config {
    delay: Timespan,
    filename: String,
    main: DatabaseConfig,
    aux: Vec<DatabaseConfig>,
}

#[derive(Debug, Clone, PartialEq)]
struct DatabaseConfig {
    host: String,
    port: Option<u16>,
    create_result: Result<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            delay: Timespan::Minutes(14),
            filename: "abcd".to_string(),
            main: DatabaseConfig {
                host: "main".to_string(),
                port: None,
                create_result: Ok("ok".to_string()),
            },
            aux: vec![
                DatabaseConfig {
                    host: "aux1".to_string(),
                    port: Some(2345),
                    create_result: Err("f1".to_string()),
                },
                DatabaseConfig {
                    host: "aux2".to_string(),
                    port: None,
                    create_result: Err("f2".to_string()),
                },
            ],
        }
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn foo() {
    let mut config = Config::default();

    let main_lens = LensImpl::<Config, DatabaseConfig>::new(|c| c.main.clone(), |c, v| c.main = v);

    assert_eq!(main_lens.get(&config), config.main);

    let port_lens = LensImpl::<DatabaseConfig, Option<u16>>::new(|c| c.port, |c, v| c.port = v);

    let composed = main_lens.compose_lens_with_lens(port_lens);
    assert_eq!(composed.get(&config), config.main.port);

    let delay_lens =
        LensImpl::<Config, Timespan, _, _>::new(|c| c.delay.clone(), |c, v| c.delay = v);

    let timespan_seconds_prism = PrismImpl::<Timespan, u32>::new(
        |c| match c {
            Timespan::Seconds(s) => Some(*s),
            _ => None,
        },
        |c, v| {
            if let Timespan::Seconds(_) = c {
                *c = Timespan::Seconds(v);
            }
        },
    );

    let delay_seconds_prism = delay_lens.compose_lens_with_prism(timespan_seconds_prism);

    let delay_prism =
        PrismImpl::<Config, Timespan, _, _>::new(|c| Some(c.delay.clone()), |c, v| c.delay = v);

    assert_eq!(delay_prism.preview(&config), Some(config.delay.clone()));
    assert_eq!(delay_seconds_prism.preview(&config), None);

    let current = delay_prism.preview(&config);
    delay_seconds_prism.set(&mut config, 10);

    assert_eq!(delay_prism.preview(&config), current);

    delay_prism.set(&mut config, Timespan::Seconds(10));

    assert_eq!(delay_seconds_prism.preview(&config), Some(10));
    delay_seconds_prism.set(&mut config, 11);

    assert_eq!(delay_seconds_prism.preview(&config), Some(11));

    let seconds_value_prism = PrismImpl::<Timespan, u32, _, _>::new(
        |c| match c {
            Timespan::Seconds(s) => Some(*s),
            _ => None,
        },
        |c, v| {
            if let Timespan::Seconds(_) = c {
                *c = Timespan::Seconds(v);
            }
        },
    );

    let delay_seconds_prism = delay_prism.compose_prism_with_prism(seconds_value_prism);
    assert_eq!(delay_seconds_prism.preview(&config), Some(11));

    let host_lens =
        LensImpl::<DatabaseConfig, String, _, _>::new(|c| c.host.clone(), |c, v| c.host = v);

    let second_database_prism = PrismImpl::<Config, DatabaseConfig, _, _>::new(
        |c| c.aux.get(1).cloned(),
        |c, v| {
            if let Some(db) = c.aux.get_mut(1) {
                *db = v;
            }
        },
    );

    let second_database_host_prism = second_database_prism.compose_prism_with_lens(host_lens);
    assert_eq!(
        second_database_host_prism.preview(&config),
        Some("aux2".to_string())
    );

    let delay_lens = LensImpl::<Config, Timespan>::new(|c| c.delay.clone(), |c, v| c.delay = v);

    let delay_iso = FallibleIsoImpl::<Timespan, u16, String>::new(
        |c| match c {
            Timespan::Seconds(s) => u16::try_from(*s).map_err(|_| "Out of bounds".to_string()),
            Timespan::Minutes(m) => u16::try_from(*m * 60).map_err(|_| "Out of bounds".to_string()),
            Timespan::Hours(h) => u16::try_from(*h * 3600).map_err(|_| "Out of bounds".to_string()),
        },
        |c: &u16| match u32::from(*c) {
            c if c % 3600 == 0 => Ok(Timespan::Hours(c / 3600)),
            c if c % 60 == 0 => Ok(Timespan::Minutes(c / 60)),
            c => Ok(Timespan::Seconds(c)),
        },
    );

    let seconds_prism = delay_lens.compose_prism_with_fallible_iso::<StringError>(delay_iso);

    assert_eq!(seconds_prism.try_get(&config), Ok(11u16));

    seconds_prism.set(&mut config, 1800u16);
    assert_eq!(delay_seconds_prism.preview(&config), None);

    let delay_lens =
        LensImpl::<Config, Timespan, _, _>::new(|c| c.delay.clone(), |c, v| c.delay = v);
    assert_eq!(delay_lens.get(&config), Timespan::Minutes(30));

    delay_lens.set(&mut config, Timespan::Hours(1000));
    assert_eq!(seconds_prism.try_get(&config), Err(NoFocus));

    let mut val: u32 = 3;

    let to_u16 = FallibleIsoImpl::<u32, u16, _, _, _>::new(
        |c| u16::try_from(*c).map_err(|_| "Too big".to_string()),
        |v| Ok(u32::from(*v)),
    );

    let times_2 = FallibleIsoImpl::<u16, u16, _, _, _>::new(
        |c| {
            let res = c.overflowing_mul(2u16);
            (!res.1).then_some(res.0).ok_or("Overflow".to_string())
        },
        |v| {
            let res = (v / 2, v % 2);
            (res.1 == 0).then_some(res.0).ok_or("Not Even".to_string())
        },
    );

    let u16_times_2 = to_u16.compose_fallible_iso_with_fallible_iso::<StringError>(times_2);

    assert_eq!(u16_times_2.try_get(&val), Ok(6u16));
    u16_times_2.set(&mut val, 4u16);
    assert_eq!(val, 2);

    assert_eq!(
        u16_times_2.try_reverse_get(&3),
        Err(StringError("Not Even".to_string()))
    );

    val = 40000;

    assert_eq!(
        u16_times_2.try_get(&val),
        Err(StringError("Overflow".to_string()))
    );

    val = 5;

    let wrapping_add_one =
        IsoImpl::<u32, u32, _, _>::new(|c| c.wrapping_add(1), |v| v.wrapping_sub(1));

    let wrapping_add_two =
        IsoImpl::<u32, u32, _, _>::new(|c| c.wrapping_add(2), |v| v.wrapping_sub(2));

    assert_eq!(wrapping_add_one.try_get(&val), Ok(6));
    wrapping_add_one.set(&mut val, 0);
    assert_eq!(val, u32::MAX);

    let wrapping_add_three = wrapping_add_one.compose_iso_with_iso(wrapping_add_two);
    assert_impl!(wrapping_add_three: Iso<u32, u32>);
    assert_eq!(wrapping_add_three.try_get(&val), Ok(2));

    let wrapping_add_one =
        IsoImpl::<u32, u32, _, _>::new(|c| c.wrapping_add(1), |v| v.wrapping_sub(1));

    let to_u16 = FallibleIsoImpl::<u32, u16, _, _, _>::new(
        |c| u16::try_from(*c).map_err(|_| "Too big".to_string()),
        |v| Ok(u32::from(*v)),
    );

    val = 65535;
    let wrapping_add_one_to_u16 =
        wrapping_add_one.compose_iso_with_fallible_iso::<StringError>(to_u16);
    assert_impl!(wrapping_add_one_to_u16: FallibleIso<u32, u16, Error = StringError>);

    assert_eq!(
        wrapping_add_one_to_u16.try_get(&val),
        Err(StringError("Too big".to_string()))
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringError(String);

impl From<Infallible> for StringError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<NoFocus> for StringError {
    fn from(_: NoFocus) -> Self {
        StringError("No focus".to_string())
    }
}

impl From<String> for StringError {
    fn from(s: String) -> Self {
        StringError(s)
    }
}

impl From<String> for NoFocus {
    fn from(_: String) -> Self {
        NoFocus
    }
}

impl From<StringError> for NoFocus {
    fn from(_: StringError) -> Self {
        NoFocus
    }
}
