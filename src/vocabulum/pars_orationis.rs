use crate::vocabulum::coniunctio::Coniunctio;

pub enum ParsOrationis<'a> {
    Nomen,
    Numerale,
    Pronomen,
    Adiectivum,
    Verbum,
    Adverbium,
    Praepositio,
    Coniunctio(&'a Coniunctio),
}
