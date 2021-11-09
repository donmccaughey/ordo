pub use crate::vocabulum::pars_orationis::ParsOrationis;
use crate::Orthographia;

mod coniunctio;
#[cfg(test)]
mod coniunctio_tests;
mod pars_orationis;

pub trait Vocabulum {
    fn formae(&self) -> Vec<&dyn Forma>;
    fn lemma(&self) -> &dyn Forma;
    fn pars_orationis(&self) -> ParsOrationis;
}

pub trait Forma {
    fn orthographia(&self) -> &Orthographia;
    fn vocabulum(&self) -> &dyn Vocabulum;
}
