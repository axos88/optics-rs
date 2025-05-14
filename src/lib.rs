#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

mod fallible_iso;
mod iso;
mod lens;
mod optic;
mod prism;
// mod traversal;
#[cfg(test)]
mod test;

pub use fallible_iso::{ComposedFallibleIso, FallibleIso, FallibleIsoImpl};
pub use iso::{ComposedIso, Iso, IsoImpl};
pub use lens::{ComposedLens, Lens, LensImpl};
pub use optic::Optic;
pub use prism::{ComposedPrism, NoFocus, Prism, PrismImpl};

/// Module containing convenience re-exports of optic composition traits.
///
/// These traits provide the combinators for chaining optics of the same type
/// (e.g. `Lens`, `Prism`, `Iso`, `FallibleIso`) in a type-safe, composable way without needing
/// to manually qualify each trait's path.
///
/// # Re-exports
///
/// - [`crate::composers::ComposableLens`]
/// - [`crate::composers::ComposablePrism`]
/// - [`crate::composers::ComposableFallibleIso`]
/// - [`crate::composers::ComposableIso`]
pub mod composers {
    pub use crate::fallible_iso::ComposableFallibleIso;
    pub use crate::iso::ComposableIso;
    pub use crate::lens::ComposableLens;
    pub use crate::prism::ComposablePrism;
}
