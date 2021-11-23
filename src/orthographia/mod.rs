use crate::errors::Irritus;
use crate::iter::char::fallible::{CharSequences, Iterators};
use crate::iter::char::Iterators as _;

mod debug;
mod display;

#[cfg(test)]
mod debug_tests;
#[cfg(test)]
mod display_tests;
#[cfg(test)]
mod mod_tests;

/// The spelling of a single form of a Latin word.
///
/// An _Orthographia_ ("spelling") is a specific spelling of one form of a
/// Latin word.  It is a string-like immutable value type that encodes the word
/// in a canonical way using a limited set of characters.  Different
/// inflections of the same word, like _canis_ ("dog") and _canes_ ("dogs") are
/// separate _Orthographiae_.  Alternate spellings, like _verto_ and _vorto_
/// ("I turn") are also separate _Orthographiae_.
///
/// ## Encoding
///
/// An _Orthographia_ can encode many features of a Latin word.  The two input
/// representations are described in detail in
/// [try_from_ascii()](Orthographia::try_from_ascii) and
/// [try_from_canonical()](Orthographia::try_from_canonical).  Both
/// representations can encode the following word features.
///
/// ### Long Vowels
///
/// An _Orthographia_ can encode long vowels, as in _hōra_ ("hour") and
/// _īnfāns_ ("infant").
///
/// ### Semivowels
///
/// Classical Latin uses the letters __I__ and __V__ as both
/// [semivowels](https://en.wikipedia.org/wiki/Semivowel) and vowels, as in the
/// words _IVLIVS_ and _VERVM_.  Over the centuries, __J__ was added to
/// represent the semivowel __I__ and __U__ to represent the vowel __V__.  In
/// its internal representation, _Orthographiae_ uses the letters __J__ and
/// __V__ strictly as semivowels and letters __I__ and __U__ strictly as vowels.
///
/// ### Proper names and lowercase
///
/// Proper names are indicated by an initial capital letter, so that the city
/// of _Ostia_ is different than _ostia_ ("doors").  Otherwise, _Orthographiae_
/// are represented using lowercase letters.
///
/// ### Compound words
///
/// Compound words can be indicated with a hyphen, like _ex-īre_ ("to go out")
/// or _duo-decim_ ("twelve").
///
/// ### Enclitics
///
/// [Suffix words](https://en.wikipedia.org/wiki/Clitic#Enclitic) like _-que_
/// ("and") are indicated by a leading hyphen.
///
/// ### Stems and endings
///
/// The division between a word stem and its ending can be indicated with a
/// vertical line (`|`) character, like _vacc|a_ ("cow").  An ending or stem by
/// itself can be indicated by a leading or trailing vertical line,
/// respectively, as in _|uum_ (4th declention genative plural ending) and
/// _senat|_ (stem of _senatus_, "senate").
///
/// ## Display
///
/// An _Orthographia_ can be formatted in a variety of ways for different
/// contexts, such as "Iūppiter", "Juppiter" or "IVPPITER".  The default
/// display format is the modern format detailed in
/// [to_modern_format()](Orthographia::to_modern_format).
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
    /// - __J__ and __V__ are used to represent semivowels only
    /// - __I__ and __U__ are used to represent vowels only
    /// - proper names start with a capital letter; all other letters are
    ///     lowercase
    /// - dipthongs like "æ" and "œ" are represented decomposed as separate
    ///     letters
    /// - a hyphen between letters indicates a compound word
    /// - a hyphen at the start indicates a suffix
    /// - a vertical line (`'|'`) between letters separates a word stem from
    ///     its ending; a second vertical line can divide complex endings, as
    ///     in `"leg|e'ba'|mus"` (_we were reading_)
    /// - a vertical line at the end indicates a word stem
    /// - a vertical line at the start indicates a word ending
    /// - spaces, punctuation and non-Latin letters (including __W__) are
    ///     prohibited
    ///
    /// Examples:
    /// - `"ma'ter"` (māter)
    /// - `"jam"` (iam)
    /// - `"i'nsul|a"` (īnsula)
    /// - `"ve'r|um"` (vērum)
    /// - `"Ju'ppiter"` (Iūppiter)
    /// - `"praeter"` (præter)
    /// - `"ex-i're"` (compound word exīre)
    /// - `"-que"` (suffix, e.g. populusque)
    /// - `"leg|is"` (legis)
    /// - `"leg|e'ba'|mus"` (legēbāmus)
    /// - `"nas|"` (stem of nasus)
    /// - `"|us"` (word ending -us)
    pub fn try_from_ascii(ascii: &str) -> Result<Orthographia, Irritus> {
        ascii
            .fallible_chars()
            .not_empty()
            .ascii_chars()
            .initial_caps()
            .long_vowel_ticks()
            .solo_hyphens()
            .solo_pipes()
            .collect::<Result<_, _>>()
            .map(|s| Orthographia { s })
    }

    /// Create an _Orthographia_ from a canonical UTF-8 representation.
    ///
    /// The canonical string must follow these rules:
    /// - long vowels are represented by precomposed Unicode letters with
    ///     macrons (`'Ā'`, `'Ē'`, etc. -- see [ordo::litterae](crate::litterae)
    ///     for the list) or by the plain vowel followed by a combining macron
    ///     character `'\u{0304}'`
    /// - __J__ and __V__ are used to represent semivowels only
    /// - __I__ and __U__ are used to represent vowels only
    /// - proper names start with a capital letter; all other letters are
    ///     lowercase
    /// - dipthongs like "æ" and "œ" are represented decomposed as separate
    ///     letters
    /// - a hyphen between letters indicates a compound word
    /// - a hyphen at the beginning indicates that this is a suffix
    /// - a vertical line (`'|'`) between letters separates a word stem from
    ///     its ending; a second vertical line can divide complex endings, as
    ///     in `"lēg|erā|mus"` (_we were reading_)
    /// - a vertical line at the end indicates a word stem
    /// - a vertical line at the start indicates a word ending
    /// - spaces, punctuation and non-Latin letters (including __W__) are
    ///     prohibited
    ///
    /// Examples:
    /// - `"ma\u{0304}ter"` (combining macron, māter)
    /// - `"jam"` (iam)
    /// - `"\u{012b}nsula"` (precomposed long 'i', īnsula)
    /// - `"ve\u{0304}rum"` (vērum)
    /// - `"Ju\u{0304}ppiter"` (Iūppiter)
    /// - `"praeter"` (præter)
    /// - `"ex-\u{012b}re"` (compound word exīre)
    /// - `"-que"` (suffix, e.g. populusque)
    /// - `"leg|is"` (legis)
    /// - `"l\u{0113}g|er\u{0101}|mus"` (lēgerāmus)
    /// - `"nas|"` (stem of nasus)
    /// - `"|us"` (word ending -us)
    pub fn try_from_canonical(canonical: &str) -> Result<Orthographia, Irritus> {
        canonical
            .fallible_chars()
            .not_empty()
            .canonical_chars()
            .initial_caps()
            .long_vowel_macrons()
            .solo_hyphens()
            .solo_pipes()
            .collect::<Result<_, _>>()
            .map(|s| Orthographia { s })
    }

    /// Format an _Orthographia_ as a String using only ASCII characters.
    ///
    /// The ASCII format preserves the distinction between __I__ and __J__ as
    /// well as __V__ and __U__.  Long vowels are followed by a single quote
    /// (`'`).  Hyphens and vertical lines are preserved.
    pub fn to_ascii_format(&self) -> String {
        self.s.chars().long_vowel_ticks().collect()
    }

    /// Format an _Orthographia_ as a String in the classical way.
    ///
    /// The classical format transforms __J__ and __U__ into __I__ and __V__
    /// respectively.  All letters are uppercase.  Long vowel marks, compound
    /// word hyphens and stem|ending separators are omitted.  Stems and word
    /// endings are indicated by `'-'` instead of  `'|'`.
    pub fn to_classical_format(&self) -> String {
        self.s
            .chars()
            .all_caps()
            .semivowel_i()
            .vowel_v()
            .no_macrons()
            .no_compound_words()
            .ending_hyphens()
            .no_stem_ending_separators()
            .stem_hyphens()
            .collect()
    }

    /// Format an _Orthographia_ as a String in the common modern way.
    ///
    /// The common modern format preserves the distinction between __V__ and
    /// __U__ but transforms __J__ into __I__; long vowel marks, compound word
    /// hyphens and stem|ending separators are omitted.  Stems and word endings
    /// are indicated by `'-'` instead of  `'|'`.
    pub fn to_modern_format(&self) -> String {
        self.s
            .chars()
            .semivowel_i()
            .no_macrons()
            .no_compound_words()
            .ending_hyphens()
            .no_stem_ending_separators()
            .stem_hyphens()
            .collect()
    }

    /// Format an _Orthographia_ as a String for teaching.
    ///
    /// The teaching format preserves the distinction between __V__ and __U__
    /// but transforms __J__ into __I__; long vowel marks are shown but
    /// compound word hyphens and stem|ending separators are omitted.  Stems
    /// and word endings are indicated by `'-'` instead of  `'|'`.
    pub fn to_teaching_format(&self) -> String {
        self.s
            .chars()
            .semivowel_i()
            .no_compound_words()
            .ending_hyphens()
            .no_stem_ending_separators()
            .stem_hyphens()
            .collect()
    }
}
