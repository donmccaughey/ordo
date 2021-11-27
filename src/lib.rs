//! Latin language utility library.
//!
//! # License
//! `ordo` is made available under a BSD-style license; see the
//! [`LICENSE`](https://github.com/donmccaughey/ordo/blob/main/LICENSE) file
//! for details.

pub mod errors;
pub mod iter;
pub mod litterae;
mod locutio;
mod numerus;
mod orthographia;
mod vocabulum;

pub use numerus::Numerus;
pub use orthographia::Orthographia;
pub use vocabulum::Forma;
pub use vocabulum::Vocabulum;
