use std::str::Chars;

use crate::iter::char::all_caps::AllCaps;
use crate::iter::char::ending_hyphens::EndingHyphens;
use crate::iter::char::long_vowel_ticks::LongVowelTicks;
use crate::iter::char::no_compound_words::NoCompoundWords;
use crate::iter::char::no_macrons::NoMacrons;
use crate::iter::char::no_stem_ending_separators::NoStemEndingSeparators;
use crate::iter::char::semivowel_i::SemivowelI;
use crate::iter::char::stem_hyphens::StemHyphens;
use crate::iter::char::vowel_v::VowelV;

/// Methods for chaining `char` iterators together.
pub trait Iterators: Iterator<Item = char> + Sized {
    fn all_caps(self) -> AllCaps<Self> {
        AllCaps::new(self)
    }

    fn ending_hyphens(self) -> EndingHyphens<Self> {
        EndingHyphens::new(self)
    }

    fn long_vowel_ticks(self) -> LongVowelTicks<Self> {
        LongVowelTicks::new(self)
    }

    fn no_compound_words(self) -> NoCompoundWords<Self> {
        NoCompoundWords::new(self)
    }

    fn no_macrons(self) -> NoMacrons<Self> {
        NoMacrons::new(self)
    }

    fn no_stem_ending_separators(self) -> NoStemEndingSeparators<Self> {
        NoStemEndingSeparators::new(self)
    }

    fn semivowel_i(self) -> SemivowelI<Self> {
        SemivowelI::new(self)
    }

    fn stem_hyphens(self) -> StemHyphens<Self> {
        StemHyphens::new(self)
    }

    fn vowel_v(self) -> VowelV<Self> {
        VowelV::new(self)
    }
}

impl<'a> Iterators for Chars<'a> {}
