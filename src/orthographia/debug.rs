use crate::orthographia::Orthographia;
use std::fmt::{Debug, Formatter};

/// Format an _Orthographia_ for debugging in the ASCII format with surrounding
/// double quotes.
///
/// See [to_ascii_format()](Orthographia::to_ascii_format) for details.
impl Debug for Orthographia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", &self.to_ascii_format())
    }
}
