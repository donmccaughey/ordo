use crate::errors::Irritus;
use crate::litterae::filters::{
    AsciiChars, CanonicalChars, InitialCaps, LongVowelMacrons, LongVowelTicks, NotEmpty,
    SoloHyphens,
};

pub trait CharFilters: Iterator<Item = Result<char, Irritus>> + Sized {
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
}
