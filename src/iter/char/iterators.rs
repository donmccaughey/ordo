use std::str::Chars;

use crate::iter::char::all_caps::AllCaps;
use crate::iter::char::consonant_i::ConsonantI;
use crate::iter::char::no_compound_words::NoCompoundWords;
use crate::iter::char::no_macrons::NoMacrons;
use crate::iter::char::vowel_v::VowelV;

/// Methods for chaining `char` iterators together.
pub trait Iterators: Iterator<Item = char> + Sized {
    fn all_caps(self) -> AllCaps<Self> {
        AllCaps::new(self)
    }

    fn consonant_i(self) -> ConsonantI<Self> {
        ConsonantI::new(self)
    }

    fn vowel_v(self) -> VowelV<Self> {
        VowelV::new(self)
    }

    fn no_compound_words(self) -> NoCompoundWords<Self> {
        NoCompoundWords::new(self)
    }

    fn no_macrons(self) -> NoMacrons<Self> {
        NoMacrons::new(self)
    }
}

impl<'a> Iterators for Chars<'a> {}
