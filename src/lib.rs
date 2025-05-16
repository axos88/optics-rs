#![doc = include_str!("../README.md")]
#![no_std]
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

mod fallible_iso;
mod getter;
mod iso;
mod lens;
mod partial_getter;
mod prism;
mod setter;
// mod traversal;

#[cfg(test)]
mod test;

pub use base::{HasGetter, HasPartialGetter, HasPartialReversible, HasReversible, HasSetter};

pub use fallible_iso::{FallibleIso, FallibleIsoImpl, composed_fallible_iso, mapped_fallible_iso};
pub use getter::{Getter, GetterImpl, composed_getter, mapped_getter};
pub use iso::{Iso, IsoImpl, composed_iso, mapped_iso};
pub use lens::{Lens, LensImpl, composed_lens, mapped_lens};
pub use partial_getter::{
    PartialGetter, PartialGetterImpl, composed_partial_getter, mapped_partial_getter,
};
pub use prism::{Prism, PrismImpl, composed_prism, mapped_prism};
pub use setter::{Setter, SetterImpl, composed_setter, mapped_setter};
