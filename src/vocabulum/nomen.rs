use crate::errors::Irritus;
use crate::vocabulum::casus::Casus;
use crate::vocabulum::declinatio::Declinatio;
use crate::vocabulum::genus::Genus;
use crate::vocabulum::numerus::Numerus;
use crate::vocabulum::{Forma, ParsOrationis};
use crate::{Orthographia, Vocabulum};
use std::pin::Pin;

pub struct Nomen {
    declinatio: Declinatio,
    formae: Vec<FormaNominis>,
    genus: Genus,
}

impl Nomen {
    pub fn try_from_ascii_stem(
        ascii_stem: &str,
        declinatio: Declinatio,
        genus: Genus,
    ) -> Result<Pin<Box<Nomen>>, Irritus> {
        let mut nomen = Box::pin(Nomen {
            declinatio,
            formae: Vec::with_capacity(10),
            genus,
        });

        if declinatio == Declinatio::Prima {
            let ascii_endings = [
                "a", "am", "ae", "ae", "a'", "ae", "a's", "a'rum", "i's", "i's",
            ];
            let mut i = 0;
            for numerus in [Numerus::Singularis, Numerus::Pluralis] {
                for casus in [
                    Casus::Nominativus,
                    Casus::Accusativus,
                    Casus::Genetivus,
                    Casus::Dativus,
                    Casus::Ablativus,
                ] {
                    let ascii = ascii_stem.to_string() + ascii_endings[i];
                    let orthographia = Orthographia::try_from_ascii(&ascii)?;
                    let nomen_ptr = &*nomen as *const Nomen;
                    nomen.formae.push(FormaNominis {
                        nomen: nomen_ptr,
                        orthographia,
                        casus,
                        numerus,
                    });
                    i += 1;
                }
            }
        } else {
            unimplemented!();
        }
        Ok(nomen)
    }
}

impl Vocabulum for Nomen {
    fn formae(&self) -> Vec<&dyn Forma> {
        self.formae.iter().map(|f| f as &dyn Forma).collect()
    }

    fn lemma(&self) -> &dyn Forma {
        &self.formae[0]
    }

    fn pars_orationis(&self) -> ParsOrationis {
        ParsOrationis::Nomen(self)
    }
}

pub struct FormaNominis {
    nomen: *const Nomen,
    orthographia: Orthographia,
    casus: Casus,
    numerus: Numerus,
}

impl Forma for FormaNominis {
    fn orthographia(&self) -> &Orthographia {
        &self.orthographia
    }

    fn vocabulum(&self) -> &dyn Vocabulum {
        unsafe { &*self.nomen }
    }
}
