use crate::errors::Irritus;
use crate::vocabulum::{Forma, ParsOrationis};
use crate::{Orthographia, Vocabulum};
use std::pin::Pin;
use std::ptr;

pub struct Coniunctio {
    forma: FormaConiunctionis,
}

impl Coniunctio {
    pub fn try_from_ascii(ascii: &str) -> Result<Pin<Box<Coniunctio>>, Irritus> {
        let orthographia = Orthographia::try_from_ascii(ascii)?;
        let mut coniunctio = Box::pin(Coniunctio {
            forma: FormaConiunctionis {
                coniunctio: ptr::null_mut(),
                orthographia,
            },
        });
        coniunctio.forma.coniunctio = &mut *coniunctio;
        Ok(coniunctio)
    }
}

impl Vocabulum for Coniunctio {
    fn formae(&self) -> Vec<&dyn Forma> {
        vec![&self.forma]
    }

    fn lemma(&self) -> &dyn Forma {
        &self.forma
    }

    fn pars_orationis(&self) -> ParsOrationis {
        ParsOrationis::Coniunctio(self)
    }
}

pub struct FormaConiunctionis {
    coniunctio: *mut Coniunctio,
    orthographia: Orthographia,
}

impl Forma for FormaConiunctionis {
    fn orthographia(&self) -> &Orthographia {
        &self.orthographia
    }

    fn vocabulum(&self) -> &dyn Vocabulum {
        unsafe { &*self.coniunctio }
    }
}
