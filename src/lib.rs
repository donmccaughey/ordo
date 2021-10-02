//! Latin language utility library.
//!
//! # License
//! `ordo` is made available under a BSD-style license; see the `LICENSE` file for details.

pub mod errors;
pub mod litterae;
#[cfg(test)]
mod litterae_tests;
mod numerus;
mod vocabulum;

pub use numerus::Numerus;
pub use vocabulum::Orthographia;
