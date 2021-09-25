use crate::errors::Nimis;
use crate::numerus::Numerus;

use std::convert::TryFrom;

impl TryFrom<u16> for Numerus {
    type Error = Nimis;

    /// Create a _Numerus_ from an integer.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let xxvi = Numerus::try_from(26).unwrap();
    /// assert_eq!("XXVI", &xxvi.to_string());
    ///
    /// let cxi: Numerus = 111.try_into().unwrap();
    /// assert_eq!("CXI", &cxi.to_string());
    /// ```
    ///
    /// If the integer is outside the range `1..=3999`, `Err(Nimis)` (_too much_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    ///
    /// let result = Numerus::try_from(0);
    /// assert!(matches!(result, Err(Nimis)));
    ///
    /// let result = Numerus::try_from(4000);
    /// assert!(matches!(result, Err(Nimis)));
    /// ```
    fn try_from(vis: u16) -> Result<Self, Self::Error> {
        if (1..=3999).contains(&vis) {
            Ok(Numerus { vis })
        } else {
            Err(Nimis)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::numerus::Numerus;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from() {
        let nihil = Numerus::try_from(0);
        assert!(nihil.is_err());

        let i = Numerus::try_from(1);
        assert!(i.is_ok());
        assert_eq!(1, i.unwrap().vis);

        let mmmcmxcix = Numerus::try_from(3999);
        assert!(mmmcmxcix.is_ok());
        assert_eq!(3999, mmmcmxcix.unwrap().vis);

        let mmmm = Numerus::try_from(4000);
        assert!(mmmm.is_err());
    }
}
