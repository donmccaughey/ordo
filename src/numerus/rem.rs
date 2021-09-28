use crate::numerus::Numerus;

use std::ops::{Rem, RemAssign};

impl Rem for Numerus {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let reliquum = self.vis % rhs.vis;
        debug_assert!(reliquum > 0);
        Numerus { vis: reliquum }
    }
}

impl Rem for &Numerus {
    type Output = Numerus;

    fn rem(self, rhs: Self) -> Self::Output {
        Numerus::rem(*self, *rhs)
    }
}

impl RemAssign for Numerus {
    fn rem_assign(&mut self, rhs: Self) {
        let reliquum = self.vis % rhs.vis;
        debug_assert!(reliquum > 0);
        self.vis = reliquum;
    }
}

impl RemAssign<&Numerus> for Numerus {
    fn rem_assign(&mut self, rhs: &Numerus) {
        Numerus::rem_assign(self, *rhs)
    }
}
