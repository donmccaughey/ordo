use crate::Numerus;

impl From<Numerus> for u16 {
    fn from(n: Numerus) -> Self {
        n.vis
    }
}
