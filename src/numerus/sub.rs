use std::ops::{Sub, SubAssign};

use crate::numerus::Numerus;

impl Sub for Numerus {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let summa = self.vis - rhs.vis;
        debug_assert!(summa > 0);
        Numerus { vis: summa }
    }
}

impl Sub for &Numerus {
    type Output = Numerus;

    fn sub(self, rhs: Self) -> Self::Output {
        Numerus::sub(*self, *rhs)
    }
}

impl SubAssign for Numerus {
    fn sub_assign(&mut self, rhs: Self) {
        let summa = self.vis - rhs.vis;
        debug_assert!(summa > 0);
        self.vis = summa;
    }
}

impl SubAssign<&Numerus> for Numerus {
    fn sub_assign(&mut self, rhs: &Numerus) {
        Numerus::sub_assign(self, *rhs);
    }
}
