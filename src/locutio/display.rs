use crate::locutio::Locutio;
use std::fmt::{Display, Formatter};

impl Display for Locutio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.orthographiae[0].to_ascii_format())?;
        for orthographia in &self.orthographiae[1..] {
            f.write_str(" ")?;
            f.write_str(&orthographia.to_ascii_format())?;
        }
        Ok(())
    }
}
