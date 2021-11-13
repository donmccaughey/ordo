use crate::errors::Irritus;
use crate::iter::char::fallible::{
    AsciiChars, CanonicalChars, InitialCaps, LongVowelMacrons, LongVowelTicks, NotEmpty,
    SoloHyphens, SoloPipes,
};

/// Methods for chaining fallible `char` iterators together.
pub trait Iterators: Iterator<Item = Result<char, Irritus>> + Sized {
    fn ascii_chars(self) -> AsciiChars<Self> {
        AsciiChars::new(self)
    }

    fn canonical_chars(self) -> CanonicalChars<Self> {
        CanonicalChars::new(self)
    }

    fn initial_caps(self) -> InitialCaps<Self> {
        InitialCaps::new(self)
    }

    fn long_vowel_macrons(self) -> LongVowelMacrons<Self> {
        LongVowelMacrons::new(self)
    }

    fn long_vowel_ticks(self) -> LongVowelTicks<Self> {
        LongVowelTicks::new(self)
    }

    fn not_empty(self) -> NotEmpty<Self> {
        NotEmpty::new(self)
    }

    fn solo_hyphens(self) -> SoloHyphens<Self> {
        SoloHyphens::new(self)
    }

    fn solo_pipes(self) -> SoloPipes<Self> {
        SoloPipes::new(self)
    }
}
