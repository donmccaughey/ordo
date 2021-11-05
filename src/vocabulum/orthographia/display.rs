use crate::vocabulum::orthographia::Orthographia;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Format an _Orthographia_ for display in the common modern way.
///
/// The common modern format uses both "v" and "u" for __V__ but only "i" for
/// __I__; long vowel marks and compound word hyphens are omitted.
impl Display for Orthographia {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let modern = self.to_modern_format();
        f.write_str(&modern)
    }
}
