use crate::Orthographia;
use crate::Vocabulum;

pub trait Forma {
    fn orthographia(&self) -> &Orthographia;
    fn vocabulum(&self) -> &dyn Vocabulum;
}
