pub use crate::vocabulum::pars_orationis::ParsOrationis;
use crate::Orthographia;

mod casus;
mod coniunctio;
mod declinatio;
mod genus;
mod nomen;
mod numerus;
mod pars_orationis;

#[cfg(test)]
mod coniunctio_tests;
#[cfg(test)]
mod nomen_tests;

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
