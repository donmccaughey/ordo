use crate::numerus::Numerus;

use std::ops::{Add, AddAssign};

impl Add for Numerus {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let summa = self.vis + rhs.vis;
        debug_assert!(summa < 4000);
        Numerus { vis: summa }
    }
}

impl Add for &Numerus {
    type Output = Numerus;

    fn add(self, rhs: Self) -> Self::Output {
        Numerus::add(*self, *rhs)
    }
}

impl AddAssign for Numerus {
    fn add_assign(&mut self, rhs: Self) {
        let summa = self.vis + rhs.vis;
        debug_assert!(summa < 4000);
        self.vis = summa;
    }
}

impl AddAssign<&Numerus> for Numerus {
    fn add_assign(&mut self, rhs: &Numerus) {
        Numerus::add_assign(self, *rhs);
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::numerus::Numerus;

    #[test]
    fn test_add() {
        let iii = Numerus::try_from(3).unwrap();
        let viii = Numerus::try_from(8).unwrap();

        let xi = iii + viii;

        assert_eq!(11, xi.vis);
    }

    #[test]
    #[should_panic]
    fn test_add_nimis() {
        let mmmcmxcix = Numerus::try_from(3999).unwrap();
        let i = Numerus::try_from(1).unwrap();

        let _ = mmmcmxcix + i;
    }

    #[test]
    fn test_add_ref() {
        let xlii = Numerus::try_from(42).unwrap();
        let xi = Numerus::try_from(11).unwrap();

        let liii = &xlii + &xi;

        assert_eq!(53, liii.vis);
    }

    #[test]
    fn test_add_assign() {
        let mut n = Numerus::try_from(11).unwrap();
        let vi = Numerus::try_from(6).unwrap();

        n += vi;

        assert_eq!(17, n.vis);
    }

    #[test]
    #[should_panic]
    fn test_add_assign_nimis() {
        let mut n = Numerus::try_from(3999).unwrap();
        let i = Numerus::try_from(1).unwrap();

        n += i;
    }

    #[test]
    fn test_add_assign_ref() {
        let mut n = Numerus::try_from(42).unwrap();
        let vi = Numerus::try_from(6).unwrap();

        n += &vi;

        assert_eq!(48, n.vis);
    }
}
