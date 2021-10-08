use std::str::Chars;

use crate::litterae::all_caps::AllCaps;
use crate::litterae::consonant_i::ConsonantI;
use crate::litterae::no_compound_words::NoCompoundWords;
use crate::litterae::no_macrons::NoMacrons;
use crate::litterae::vowel_v::VowelV;

pub trait CharTransforms: Iterator<Item = char> + Sized {
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

impl<'a> CharTransforms for Chars<'a> {}
