use crate::HasGetter;
use crate::HasPartialGetter;
use crate::HasSetter;
use crate::lens::{Lens, mapped_lens};
use crate::prism::{Prism, mapped_prism};
use crate::{FallibleIso, HasPartialReversible, Iso, mapped_fallible_iso, mapped_iso};
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

macro_rules! assert_impl {
    ($val:ident : $trait:path) => {{
        fn _assert_impl<T: $trait>(_v: &T) {}
        _assert_impl(&$val);
    }};
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum Timespan {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Config {
    delay: Timespan,
    filename: String,
    main: DatabaseConfig,
    aux: Vec<DatabaseConfig>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
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
fn read_value_using_lens() {
    let config = Config::default();
    let main_lens = mapped_lens(|c: &Config| c.main.clone(), |c, v| c.main = v);

    assert_eq!(main_lens.get(&config), config.main);
}

#[test]
fn can_comnbine_lenses() {
    let mut config = Config::default();

    let main_lens = mapped_lens(|c: &Config| c.main.clone(), |c, v| c.main = v);
    let port_lens = mapped_lens(|c: &DatabaseConfig| c.port, |c, v| c.port = v);
    let composed = main_lens.compose_with_lens(port_lens);
    assert_impl!(composed: Lens<Config, Option<u16>>);

    assert_eq!(composed.get(&config), config.main.port);

    composed.set(&mut config, Some(42));
    assert_eq!(composed.get(&config), Some(42));
}

#[test]
fn read_value_using_prism() {
    let config = Config::default();
    let main_prism = mapped_prism(|c: &Config| Ok::<_, ()>(c.main.clone()), |c, v| c.main = v);
    assert_eq!(main_prism.try_get(&config), Ok(config.main));
}

#[test]
fn can_combine_prisms() {
    let mut config = Config::default();

    let main_prism = mapped_prism(|c: &Config| Ok::<_, ()>(c.main.clone()), |c, v| c.main = v);
    let port_prism = mapped_prism(
        |c: &DatabaseConfig| c.port.ok_or(()),
        |c, v| c.port = Some(v),
    );
    let composed = main_prism.compose_with_prism(port_prism);

    assert_impl!(composed: Prism<Config, u16>);
    assert_eq!(composed.try_get(&config).ok(), config.main.port);

    composed.set(&mut config, 42);
    assert_eq!(composed.try_get(&config), Ok::<_, ()>(42));
}

#[test]
fn can_combine_lens_with_prisms() {
    let config = Config::default();

    let main_lens = mapped_lens(|c: &Config| c.main.clone(), |c, v| c.main = v);
    let port_lens = mapped_lens(|c: &DatabaseConfig| c.port, |c, v| c.port = v);
    let main_prism = mapped_prism(|c: &Config| Ok::<_, ()>(c.main.clone()), |c, v| c.main = v);
    let port_prism = mapped_prism(|c: &DatabaseConfig| Ok::<_, ()>(c.port), |c, v| c.port = v);

    let composed1 = main_lens.compose_with_prism(port_prism);
    let composed2 = main_prism.compose_with_lens(port_lens);

    assert_impl!(composed1: Prism<Config, Option<u16>>);
    assert_impl!(composed1: Prism<Config, Option<u16>>);

    assert_eq!(composed1.try_get(&config), Ok(config.main.port));
    assert_eq!(composed2.try_get(&config), Ok(config.main.port));
}

#[test]
#[allow(clippy::too_many_lines)]
fn foo() {
    let mut config = Config::default();

    let main_lens = mapped_lens(|c: &Config| c.main.clone(), |c, v| c.main = v);

    assert_eq!(main_lens.get(&config), config.main);

    let port_lens = mapped_lens(|c: &DatabaseConfig| c.port, |c, v| c.port = v);

    let composed = main_lens.compose_with_lens(port_lens);
    assert_eq!(composed.get(&config), config.main.port);

    let delay_lens = mapped_lens(|c: &Config| c.delay.clone(), |c, v| c.delay = v);

    let timespan_seconds_prism = mapped_prism(
        |c: &Timespan| match *c {
            Timespan::Seconds(s) => Ok(s),
            _ => Err(()),
        },
        |c, v| {
            if let Timespan::Seconds(_) = c {
                *c = Timespan::Seconds(v);
            }
        },
    );

    let delay_seconds_prism = delay_lens.compose_with_prism(timespan_seconds_prism);

    let delay_prism = mapped_prism(|c: &Config| Ok(c.delay.clone()), |c, v| c.delay = v);

    assert_eq!(delay_prism.try_get(&config), Ok(config.delay.clone()));
    assert_eq!(delay_seconds_prism.try_get(&config), Err(()));

    let current = delay_prism.try_get(&config);
    delay_seconds_prism.set(&mut config, 10);

    assert_eq!(delay_prism.try_get(&config), current);

    delay_prism.set(&mut config, Timespan::Seconds(10));

    assert_eq!(delay_seconds_prism.try_get(&config), Ok(10));
    delay_seconds_prism.set(&mut config, 11);

    assert_eq!(delay_seconds_prism.try_get(&config), Ok(11));

    let seconds_value_prism = mapped_prism(
        |c| match c {
            Timespan::Seconds(s) => Ok(*s),
            _ => Err(()),
        },
        |c, v| {
            if let Timespan::Seconds(_) = c {
                *c = Timespan::Seconds(v);
            }
        },
    );

    let delay_seconds_prism = delay_prism.compose_with_prism(seconds_value_prism);
    assert_eq!(delay_seconds_prism.try_get(&config), Ok(11));

    let host_lens = mapped_lens(|c: &DatabaseConfig| c.host.clone(), |c, v| c.host = v);

    let second_database_prism = mapped_prism(
        |c: &Config| c.aux.get(1).cloned().ok_or(()),
        |c, v| {
            if let Some(db) = c.aux.get_mut(1) {
                *db = v;
            }
        },
    );

    let second_database_host_prism = second_database_prism.compose_with_lens(host_lens);
    assert_eq!(
        second_database_host_prism.try_get(&config),
        Ok("aux2".to_string())
    );

    let delay_lens = mapped_lens(|c: &Config| c.delay.clone(), |c, v| c.delay = v);

    let delay_iso = mapped_fallible_iso(
        |c| match c {
            Timespan::Seconds(s) => u16::try_from(*s).map_err(|_| "Out of bounds".to_string()),
            Timespan::Minutes(m) => u16::try_from(*m * 60).map_err(|_| "Out of bounds".to_string()),
            Timespan::Hours(h) => u16::try_from(*h * 3600).map_err(|_| "Out of bounds".to_string()),
        },
        |c: &u16| match u32::from(*c) {
            c if c % 3600 == 0 => Ok::<_, ()>(Timespan::Hours(c / 3600)),
            c if c % 60 == 0 => Ok::<_, ()>(Timespan::Minutes(c / 60)),
            c => Ok::<_, ()>(Timespan::Seconds(c)),
        },
    );

    let seconds_prism = delay_lens.compose_with_fallible_iso(delay_iso);

    assert_eq!(seconds_prism.try_get(&config), Ok(11u16));

    seconds_prism.set(&mut config, 1800u16);
    assert_eq!(delay_seconds_prism.try_get(&config), Err(()));

    let delay_lens = mapped_lens(|c: &Config| c.delay.clone(), |c, v| c.delay = v);
    assert_eq!(delay_lens.get(&config), Timespan::Minutes(30));

    delay_lens.set(&mut config, Timespan::Hours(1000));
    assert_eq!(
        seconds_prism.try_get(&config),
        Err("Out of bounds".to_string())
    );

    let mut val: u32 = 3;

    let to_u16 = mapped_fallible_iso(
        |c| u16::try_from(*c).map_err(|_| "Too big".to_string()),
        |v| Ok::<_, String>(u32::from(*v)),
    );

    let times_2 = mapped_fallible_iso(
        |c: &u16| {
            let res = c.overflowing_mul(2u16);
            (!res.1).then_some(res.0).ok_or("Overflow".to_string())
        },
        |v| {
            let res = (v / 2, v % 2);
            (res.1 == 0).then_some(res.0).ok_or("Not Even".to_string())
        },
    );

    let u16_times_2 = to_u16.compose_with_fallible_iso(times_2);

    assert_eq!(u16_times_2.try_get(&val), Ok(6u16));
    u16_times_2.set(&mut val, 4u16);
    assert_eq!(val, 2);

    assert_eq!(u16_times_2.try_reverse_get(&3), Err("Not Even".to_string()));

    val = 40000;

    assert_eq!(u16_times_2.try_get(&val), Err("Overflow".to_string()));

    val = 5;

    let wrapping_add_one = mapped_iso(|c: &u32| c.wrapping_add(1), |v| v.wrapping_sub(1));

    let wrapping_add_two = mapped_iso(|c: &u32| c.wrapping_add(2), |v| v.wrapping_sub(2));

    assert_eq!(wrapping_add_one.try_get(&val), Ok(6));
    wrapping_add_one.set(&mut val, 0);
    assert_eq!(val, u32::MAX);

    let wrapping_add_three = wrapping_add_one.compose_with_iso(wrapping_add_two);
    assert_impl!(wrapping_add_three: Iso<u32, u32>);
    assert_eq!(wrapping_add_three.try_get(&val), Ok(2));

    let wrapping_add_one = mapped_iso(|c: &u32| c.wrapping_add(1), |v| v.wrapping_sub(1));

    let to_u16 = mapped_fallible_iso(
        |c| u16::try_from(*c).map_err(|_| "Too big".to_string()),
        |v| Ok::<_, String>(u32::from(*v)),
    );

    val = 65535;
    let wrapping_add_one_to_u16 = wrapping_add_one.compose_with_fallible_iso(to_u16);
    assert_impl!(wrapping_add_one_to_u16: FallibleIso<u32, u16>);

    assert_eq!(
        wrapping_add_one_to_u16.try_get(&val),
        Err("Too big".to_string())
    );
}
