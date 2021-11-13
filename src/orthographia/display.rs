use crate::orthographia::Orthographia;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Format an _Orthographia_ for display in the common modern way.
///
/// See [to_modern_format()](Orthographia::to_modern_format) for details.
impl Display for Orthographia {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_modern_format())
    }
}
