use crate::locutio::Locutio;
use std::fmt::{Debug, Formatter};

impl Debug for Locutio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\"")?;
        f.write_str(&self.orthographiae[0].to_ascii_format())?;
        for orthographia in &self.orthographiae[1..] {
            f.write_str(" ")?;
            f.write_str(&orthographia.to_ascii_format())?;
        }
        f.write_str("\"")
    }
}
