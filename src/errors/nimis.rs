use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// "Too big" ([_nimis_](https://logeion.uchicago.edu/nimis)) error used by
/// [`TryFrom`](std::convert::TryFrom) for [`Numerus`](crate::Numerus).
///
/// Also returned for zero or smaller values.
#[derive(Debug)]
pub struct Nimis;

impl Display for Nimis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "nimis")
    }
}

impl error::Error for Nimis {}
