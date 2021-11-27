#[cfg(test)]
mod mod_tests;

use crate::Orthographia;

pub struct Locutio {
    orthographiae: Vec<Orthographia>,
}

impl Locutio {
    pub fn from_word(orthographia: Orthographia) -> Locutio {
        Locutio {
            orthographiae: vec![orthographia],
        }
    }

    pub fn new(orthographiae: Vec<Orthographia>) -> Locutio {
        Locutio { orthographiae }
    }
}
