#![doc = include_str!("../README.md")]
#![no_std]
// #![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

pub(crate) mod fallible_iso;
pub(crate) mod iso;
pub(crate) mod lens;
pub(crate) mod optic;
pub(crate) mod prism;
// mod traversal;
#[cfg(test)]
mod test;

pub use fallible_iso::{
    ComposeWithFallibleIso, FallibleIso, FallibleIsoImpl, composed_fallible_iso,
    mapped_fallible_iso,
};
pub use iso::{ComposeWithIso, Iso, IsoImpl, composed_iso, mapped_iso};
pub use lens::{ComposeWithLens, Lens, LensImpl, composed_lens, mapped_lens};
pub use optic::Optic;
pub use prism::{ComposeWithPrism, NoFocus, Prism, PrismImpl, composed_prism, mapped_prism};
