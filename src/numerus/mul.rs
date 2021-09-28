use crate::numerus::Numerus;

use std::ops::{Mul, MulAssign};

impl Mul for Numerus {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let summa = self.vis * rhs.vis;
        debug_assert!(summa < 4000);
        Numerus { vis: summa }
    }
}

impl Mul for &Numerus {
    type Output = Numerus;

    fn mul(self, rhs: Self) -> Self::Output {
        Numerus::mul(*self, *rhs)
    }
}

impl MulAssign for Numerus {
    fn mul_assign(&mut self, rhs: Self) {
        let summa = self.vis * rhs.vis;
        debug_assert!(summa < 4000);
        self.vis = summa;
    }
}

impl MulAssign<&Numerus> for Numerus {
    fn mul_assign(&mut self, rhs: &Numerus) {
        Numerus::mul_assign(self, *rhs)
    }
}
