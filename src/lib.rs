#![doc = include_str!("../README.md")]
// #![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

use core::convert::Infallible;

fn infallible<E>(e: Infallible) -> E {
    match e {}
}

mod base;
mod extensions;
mod optics;

#[cfg(test)]
mod test;

pub use base::{HasGetter, HasReverseGet, HasSetter};
pub use extensions::{HasOver, HasTotalGetter, HasTotalReverseGet};

pub use optics::fallible_iso::{
    FallibleIso, FallibleIsoImpl, composed_fallible_iso, identity_fallible_iso, mapped_fallible_iso,
};
pub use optics::getter::{Getter, GetterImpl, composed_getter, identity_getter, mapped_getter};
pub use optics::iso::{Iso, IsoImpl, composed_iso, identity_iso, mapped_iso};
pub use optics::lens::{Lens, LensImpl, composed_lens, identity_lens, mapped_lens};
pub use optics::partial_getter::{
    PartialGetter, PartialGetterImpl, composed_partial_getter, identity_partial_getter,
    mapped_partial_getter,
};
pub use optics::prism::{Prism, PrismImpl, composed_prism, identity_prism, mapped_prism};
pub use optics::setter::{Setter, SetterImpl, composed_setter, identity_setter, mapped_setter};
