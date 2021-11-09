use crate::vocabulum::coniunctio::Coniunctio;
use crate::vocabulum::nomen::Nomen;

pub enum ParsOrationis<'a> {
    Nomen(&'a Nomen),
    Numerale,
    Pronomen,
    Adiectivum,
    Verbum,
    Adverbium,
    Praepositio,
    Coniunctio(&'a Coniunctio),
}
