//! Latin language utility library.
//!
//! # License
//! `ordo` is made available under a BSD-style license; see the `LICENSE` file for details.

pub mod errors;
mod forma;
pub mod iter;
pub mod litterae;
mod numerus;
mod orthographia;
mod vocabulum;

pub use forma::Forma;
pub use numerus::Numerus;
pub use orthographia::Orthographia;
pub use vocabulum::Vocabulum;
