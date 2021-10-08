use crate::errors::Irritus;
use crate::litterae::*;

mod debug;
#[cfg(test)]
mod debug_tests;
mod default;
#[cfg(test)]
mod default_tests;
mod display;
#[cfg(test)]
mod display_tests;
#[cfg(test)]
mod mod_tests;

/// A single form of a Latin word.
///
/// An _Orthographia_ ("spelling") is a specific spelling of one form of a
/// Latin word.  It is a string-like immutable value type that encodes the word
/// in a canonical way using a limited set of characters.  Different
/// inflections of the same word, like _canis_ ("dog") and _canes_ ("dogs") are
/// separate _Orthographiae_.  Alternate spellings, like _verto_ and _vorto_
/// ("I turn") are also separate _Orthographiae_.
///
/// An _Orthographia_ can encode long vowels, as in _hōra_ ("hour") and
/// _īnfāns_ ("infant").  Consonant / semivowel uses of __I__ and __V__ can be
/// distinguished from vowels, as in the first letters of _Iulius_ ("Julius")
/// and _vērum_ ("truth").  Proper names are indicated by an initial capital
/// letter, so that the city of _Ostia_ is different than _ostia_ ("doors").
/// Compound words can be indicated with a hyphen, like _ex-īre_ ("to go out")
/// or _duo-decim_ ("twelve").
///
/// An _Orthographia_ can be formatted in a variety of ways for different
/// contexts, such as "Iūppiter", "Juppiter" or "IVPPITER".  The default
/// display format uses both "v" and "u" for __V__ but only "i" for __I__; long
/// vowel marks and compound word hyphens are omitted.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Orthographia {
    s: String,
}

impl Orthographia {
    /// Create an _Orthographia_ from an ASCII representation.
    ///
    /// The ASCII string must follow these rules:
    /// - long vowels are followed by a single quote (`'`) instead of a
    ///     [macron](https://en.wikipedia.org/wiki/Macron_(diacritic)) or an
    ///     [apex](https://en.wikipedia.org/wiki/Apex_(diacritic))
    /// - `'j'` and `'v'` are used when __I__ and __V__ are consonants / semivowels
    /// - `'i'` and `'u'` are used when __I__ and __V__ represent vowels
    /// - proper names start with a capital letter; all other letters are lower case
    /// - dipthongs like "ae" and "oe" are represented decomposed as separate letters
    /// - a hyphen between letters indicates a compound word
    /// - a hyphen at the end indicates that this is a word stem
    /// - a hyphen at the beginning indicates that this is a suffix
    /// - spaces, punctuation and non-Latin letters (including 'W') are prohibited
    ///
    /// Examples:
    /// - "ma'ter" (māter)
    /// - "jam" (iam)
    /// - "i'nsula" (īnsula)
    /// - "ve'rum" (vērum)
    /// - "Ju'ppiter" (Iūppiter)
    /// - "praeter" (præter)
    /// - "ex-i're" (compound word exīre)
    /// - "magn-" (stem of magnus)
    /// - "-que" (suffix, e.g. populusque)
    pub fn try_from_ascii(ascii: &str) -> Result<Orthographia, Irritus> {
        if ascii.is_empty() {
            return Err(Irritus);
        }
        let mut s = String::with_capacity(ascii.len());
        let mut prior: Option<char> = None;
        for ch in ascii.chars() {
            match ch {
                '-' => {
                    if let Some('-') = prior {
                        return Err(Irritus);
                    }
                    s.push(ch)
                }
                'A'..='V' | 'X'..='Z' => {
                    if prior.is_some() {
                        return Err(Irritus);
                    }
                    s.push(ch)
                }
                'a'..='v' | 'x'..='z' => s.push(ch),
                '\'' => match prior {
                    None => return Err(Irritus),
                    Some(prior) => {
                        if is_short_vowel(prior) {
                            s.pop();
                            s.push(to_long_vowel(prior));
                        } else {
                            return Err(Irritus);
                        }
                    },
                },
                _ => return Err(Irritus),
            }
            prior = Some(ch);
        }
        Ok(Orthographia { s })
    }

    /// Create an _Orthographia_ from a canonical UTF-8 representation.
    ///
    /// The canonical string must follow these rules:
    /// - long vowels are represented by precomposed Unicode letters with
    ///     macrons (`'Ā'`, `'Ē'`, etc. -- see [crate::litterae] for the list)
    ///     or by the plain vowel followed by a combining macron character
    ///     `'\u{0304}'`
    /// - `'j'` and `'v'` are used where __I__ and __V__ represent consonants or semivowels
    /// - `'i'` and `'u'` are used where __I__ and __V__ represent vowels
    /// - proper names start with a capital letter; all other letters are lower case
    /// - dipthongs like "ae" and "oe" are represented decomposed as separate letters
    /// - a hyphen between letters indicates a compound word
    /// - a hyphen at the end indicates that this is a word stem
    /// - a hyphen at the beginning indicates that this is a suffix
    /// - spaces, punctuation and non-Latin letters (including 'W') are prohibited
    ///
    /// Examples:
    /// - "ma\u{0304}ter" (combining macron, māter)
    /// - "jam" (iam)
    /// - "\u{012b}nsula" (precomposed long 'i', īnsula)
    /// - "ve\u{0304}rum" (vērum)
    /// - "Ju\u{0304}ppiter" (Iūppiter)
    /// - "praeter" (præter)
    /// - "ex-\u{012b}re" (compound word exīre)
    /// - "magn-" (stem of magnus)
    /// - "-que" (suffix, e.g. populusque)
    pub fn try_from_canonical(canonical: &str) -> Result<Orthographia, Irritus> {
        if canonical.is_empty() {
            return Err(Irritus);
        }
        let mut s = String::with_capacity(canonical.len());
        let mut prior: Option<char> = None;
        for ch in canonical.chars() {
            match ch {
                '-' => {
                    if let Some('-') = prior {
                        return Err(Irritus);
                    }
                    s.push(ch)
                }
                'A'..='V' | 'X'..='Z' => {
                    if prior.is_some() {
                        return Err(Irritus);
                    }
                    s.push(ch)
                }
                'a'..='v' | 'x'..='z' => s.push(ch),
                CAPITAL_LONG_A | CAPITAL_LONG_E | CAPITAL_LONG_I | CAPITAL_LONG_O
                | CAPITAL_LONG_U | CAPITAL_LONG_Y | SMALL_LONG_A | SMALL_LONG_E | SMALL_LONG_I
                | SMALL_LONG_O | SMALL_LONG_U | SMALL_LONG_Y => s.push(ch),
                MACRON => match prior {
                    None => return Err(Irritus),
                    Some(prior) => {
                        if is_short_vowel(prior) {
                            s.pop();
                            s.push(to_long_vowel(prior));
                        } else {
                            return Err(Irritus);
                        }
                    },
                },
                _ => return Err(Irritus),
            }
            prior = Some(ch);
        }
        Ok(Orthographia { s })
    }

    /// Format an _Orthographia_ as a String in the classical way.
    ///
    /// The classical format uses only "v" for __V__ and only "i" for __I__;
    /// only capital letters are used.  Long vowel marks and compound word
    /// hyphens are omitted.
    pub fn to_classical_format(&self) -> String {
        self.s
            .chars()
            .all_caps()
            .consonant_i()
            .vowel_v()
            .no_macrons()
            .no_compound_words()
            .collect()
    }

    /// Format an _Orthographia_ as a String in the common modern way.
    ///
    /// The common modern format uses both "v" and "u" for __V__ but only "i"
    /// for __I__; long vowel marks and compound word hyphens are omitted.
    pub fn to_modern_format(&self) -> String {
        self.s
            .chars()
            .consonant_i()
            .no_macrons()
            .no_compound_words()
            .collect()
    }

    /// Format an _Orthographia_ as a String for teaching.
    ///
    /// The teaching format uses both "v" and "u" for __V__ but only "i"
    /// for __I__; long vowel marks are shown but compound word hyphens are
    /// omitted.
    pub fn to_teaching_format(&self) -> String {
        self.s.chars().consonant_i().no_compound_words().collect()
    }
}
