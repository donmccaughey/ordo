use std::fmt;
use std::fmt::{Display, Formatter};

use crate::Numerus;

impl Display for Numerus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self < &Numerus::MIN {
            return write!(f, "nihil ({})", self.vis);
        }
        if self > &Numerus::MAX {
            return write!(f, "nimis ({})", self.vis);
        }

        let mut s = String::default();
        let mut reliquum = self.vis;

        let mut minue = |summa: u16, litterae: &str| {
            while reliquum >= summa {
                s += litterae;
                reliquum -= summa;
            }
        };

        minue(1000, "M");
        minue(900, "CM");
        minue(500, "D");
        minue(400, "CD");
        minue(100, "C");
        minue(90, "XC");
        minue(50, "L");
        minue(40, "XL");
        minue(10, "X");
        minue(9, "IX");
        minue(5, "V");
        minue(4, "IV");
        minue(1, "I");

        debug_assert_eq!(0, reliquum);

        f.write_str(&s)
    }
}
