//! Custom `char` iterators that extend [std::str::Chars].

pub use all_caps::AllCaps;
pub use consonant_i::ConsonantI;
pub use iterators::Iterators;
pub use no_compound_words::NoCompoundWords;
pub use no_macrons::NoMacrons;
pub use vowel_v::VowelV;

mod all_caps;
mod consonant_i;
pub mod fallible;
mod iterators;
mod no_compound_words;
mod no_macrons;
mod vowel_v;
