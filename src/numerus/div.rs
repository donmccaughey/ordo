use crate::numerus::Numerus;

use std::ops::{Div, DivAssign};

impl Div for Numerus {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let summa = self.vis / rhs.vis;
        debug_assert!(summa > 0);
        Numerus { vis: summa }
    }
}

impl Div for &Numerus {
    type Output = Numerus;

    fn div(self, rhs: Self) -> Self::Output {
        Numerus::div(*self, *rhs)
    }
}

impl DivAssign for Numerus {
    fn div_assign(&mut self, rhs: Self) {
        let summa = self.vis / rhs.vis;
        debug_assert!(summa > 0);
        self.vis = summa;
    }
}

impl DivAssign<&Numerus> for Numerus {
    fn div_assign(&mut self, rhs: &Numerus) {
        Numerus::div_assign(self, *rhs)
    }
}
