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
  composed_fallible_iso, identity_fallible_iso, mapped_fallible_iso, FallibleIso, FallibleIsoImpl,
};
pub use optics::getter::{composed_getter, identity_getter, mapped_getter, Getter, GetterImpl};
pub use optics::iso::{composed_iso, identity_iso, mapped_iso, Iso, IsoImpl};
pub use optics::lens::{composed_lens, identity_lens, mapped_lens, Lens, LensImpl};
pub use optics::partial_getter::{
  composed_partial_getter, identity_partial_getter, mapped_partial_getter, PartialGetter,
  PartialGetterImpl,
};
pub use optics::prism::{composed_prism, identity_prism, mapped_prism, Prism, PrismImpl};
pub use optics::setter::{composed_setter, identity_setter, mapped_setter, Setter, SetterImpl};
