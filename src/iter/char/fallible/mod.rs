//! Custom fallible iterators for `char` sequences.

pub use ascii_chars::AsciiChars;
pub use canonical_chars::CanonicalChars;
pub use char_filter::CharFilter;
pub use filter::Filter;
pub use initial_caps::InitialCaps;
pub use iterators::Iterators;
pub use long_vowel_macrons::LongVowelMacrons;
pub use long_vowel_ticks::LongVowelTicks;
pub use not_empty::NotEmpty;
pub use solo_hyphen::SoloHyphens;
pub use solo_pipes::SoloPipes;

mod ascii_chars;
mod canonical_chars;
mod char_filter;
mod filter;
mod initial_caps;
mod iterators;
mod long_vowel_macrons;
mod long_vowel_ticks;
mod not_empty;
mod solo_hyphen;
mod solo_pipes;

#[cfg(test)]
mod ascii_chars_tests;
#[cfg(test)]
mod canonical_chars_tests;
#[cfg(test)]
mod initial_caps_tests;
#[cfg(test)]
mod long_vowel_macrons_tests;
#[cfg(test)]
mod long_vowel_ticks_tests;
#[cfg(test)]
mod not_empty_tests;
#[cfg(test)]
mod solo_hyphens_tests;
#[cfg(test)]
mod solo_pipes_tests;
