use crate::orthographia::Orthographia;
use std::fmt::{Debug, Formatter};

/// Format an _Orthographia_ for debugging in the ASCII format with surrounding
/// double quotes.
///
/// The ASCII format uses both "i" and "j" for __I__ as well as "v" and
/// "u" for __V__.  Long vowels are followed by a single quote (`'`) and
/// hyphens are preserved.
impl Debug for Orthographia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", &self.to_ascii_format())
    }
}
