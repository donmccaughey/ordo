mod debug;
mod display;

#[cfg(test)]
mod debug_tests;
#[cfg(test)]
mod display_tests;
#[cfg(test)]
mod mod_tests;

use crate::errors::Irritus;
use crate::Orthographia;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Locutio {
    orthographiae: Vec<Orthographia>,
}

impl Locutio {
    pub fn one_word(orthographia: Orthographia) -> Locutio {
        Locutio {
            orthographiae: vec![orthographia],
        }
    }

    pub fn two_words(orthographia1: Orthographia, orthographia2: Orthographia) -> Locutio {
        Locutio {
            orthographiae: vec![orthographia1, orthographia2],
        }
    }

    pub fn try_new(orthographiae: Vec<Orthographia>) -> Result<Locutio, Irritus> {
        if orthographiae.is_empty() {
            Err(Irritus)
        } else {
            Ok(Locutio { orthographiae })
        }
    }
}
