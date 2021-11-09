use crate::orthographia::Orthographia;

impl Default for Orthographia {
    fn default() -> Self {
        Orthographia::try_from_ascii("nu'llus").unwrap()
    }
}
