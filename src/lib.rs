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

mod fallible_iso;
mod iso;
mod lens;
mod prism;
// mod traversal;

mod getter;
mod partial_getter;
mod partial_reversible;
mod reversible;
mod setter;
#[cfg(test)]
mod test;

pub use fallible_iso::{FallibleIso, FallibleIsoImpl, composed_fallible_iso, mapped_fallible_iso};
pub use getter::Getter;
pub use iso::{Iso, IsoImpl, composed_iso, mapped_iso};
pub use lens::{Lens, LensImpl, composed_lens, mapped_lens};
pub use partial_getter::PartialGetter;
pub use partial_reversible::PartialReversible;
pub use prism::{Prism, PrismImpl, composed_prism, mapped_prism};
pub use reversible::Reversible;
pub use setter::Setter;
