use crate::verbum::Verbum;

impl Default for Verbum {
    fn default() -> Self {
        Verbum::try_from_ascii("nu'llus").unwrap()
    }
}
