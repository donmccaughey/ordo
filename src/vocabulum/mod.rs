use crate::vocabulum::pars_orationis::ParsOrationis;
use crate::Forma;

mod pars_orationis;

pub trait Vocabulum {
    fn formae(&self) -> Vec<&dyn Forma>;
    fn lemma(&self) -> &dyn Forma;
    fn pars_orationis(&self) -> ParsOrationis;
}
