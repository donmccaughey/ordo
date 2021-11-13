//! Custom `char` iterators that extend [std::str::Chars].

pub use all_caps::AllCaps;
pub use consonant_i::ConsonantI;
pub use ending_hyphens::EndingHyphens;
pub use iterators::Iterators;
pub use long_vowel_ticks::LongVowelTicks;
pub use no_compound_words::NoCompoundWords;
pub use no_macrons::NoMacrons;
pub use no_stem_ending_separators::NoStemEndingSeparators;
pub use stem_hyphens::StemHyphens;
pub use vowel_v::VowelV;

mod all_caps;
mod consonant_i;
mod ending_hyphens;
pub mod fallible;
mod iterators;
mod long_vowel_ticks;
mod no_compound_words;
mod no_macrons;
mod no_stem_ending_separators;
mod stem_hyphens;
mod vowel_v;

#[cfg(test)]
mod long_vowel_ticks_tests;
