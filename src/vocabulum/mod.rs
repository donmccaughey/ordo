pub use crate::vocabulum::pars_orationis::ParsOrationis;
use crate::Orthographia;

mod casus;
mod coniunctio;
#[cfg(test)]
mod coniunctio_tests;
mod declinatio;
mod genus;
mod nomen;
#[cfg(test)]
mod nomen_tests;
mod numerus;
mod pars_orationis;

/// A Latin word with all its forms.
pub trait Vocabulum {
    fn formae(&self) -> Vec<&dyn Forma>;
    fn lemma(&self) -> &dyn Forma;
    fn pars_orationis(&self) -> ParsOrationis;
}

/// A particular form of a Latin word.
pub trait Forma {
    fn orthographia(&self) -> &Orthographia;
    fn vocabulum(&self) -> &dyn Vocabulum;
}
