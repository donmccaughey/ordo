use crate::errors::Nimis;
use crate::numerus::Numerus;

use std::convert::{TryFrom, TryInto};

impl TryFrom<i8> for Numerus {
    type Error = Nimis;

    /// Create a _Numerus_ from an i8 integer.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let xxvi = Numerus::try_from(26_i8).unwrap();
    /// assert_eq!("XXVI", &xxvi.to_string());
    ///
    /// let cxi: Numerus = 111_i8.try_into().unwrap();
    /// assert_eq!("CXI", &cxi.to_string());
    /// ```
    ///
    /// If the integer is outside the range `1..=3999`, `Err(Nimis)` (_too much_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    ///
    /// let result = Numerus::try_from(-128_i8);
    /// assert!(matches!(result, Err(Nimis)));
    ///
    /// let result = Numerus::try_from(0_i8);
    /// assert!(matches!(result, Err(Nimis)));
    /// ```
    fn try_from(vis: i8) -> Result<Self, Self::Error> {
        if vis < 1 {
            Err(Nimis)
        } else {
            Ok(Numerus {
                vis: vis.try_into().unwrap(),
            })
        }
    }
}

impl TryFrom<i16> for Numerus {
    type Error = Nimis;

    /// Create a _Numerus_ from an i16 integer.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let xxvi = Numerus::try_from(26_i16).unwrap();
    /// assert_eq!("XXVI", &xxvi.to_string());
    ///
    /// let cxi: Numerus = 111_i16.try_into().unwrap();
    /// assert_eq!("CXI", &cxi.to_string());
    /// ```
    ///
    /// If the integer is outside the range `1..=3999`, `Err(Nimis)` (_too much_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    ///
    /// let result = Numerus::try_from(0_i16);
    /// assert!(matches!(result, Err(Nimis)));
    ///
    /// let result = Numerus::try_from(4000_i16);
    /// assert!(matches!(result, Err(Nimis)));
    /// ```
    fn try_from(vis: i16) -> Result<Self, Self::Error> {
        if (1..=3999).contains(&vis) {
            Ok(Numerus {
                vis: vis.try_into().unwrap(),
            })
        } else {
            Err(Nimis)
        }
    }
}

impl TryFrom<i32> for Numerus {
    type Error = Nimis;

    /// Create a _Numerus_ from an i32 integer.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let xxvi = Numerus::try_from(26_i32).unwrap();
    /// assert_eq!("XXVI", &xxvi.to_string());
    ///
    /// let cxi: Numerus = 111_i32.try_into().unwrap();
    /// assert_eq!("CXI", &cxi.to_string());
    /// ```
    ///
    /// If the integer is outside the range `1..=3999`, `Err(Nimis)` (_too much_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    ///
    /// let result = Numerus::try_from(0_i32);
    /// assert!(matches!(result, Err(Nimis)));
    ///
    /// let result = Numerus::try_from(4000_i32);
    /// assert!(matches!(result, Err(Nimis)));
    /// ```
    fn try_from(vis: i32) -> Result<Self, Self::Error> {
        if (1..=3999).contains(&vis) {
            Ok(Numerus {
                vis: vis.try_into().unwrap(),
            })
        } else {
            Err(Nimis)
        }
    }
}

impl TryFrom<u8> for Numerus {
    type Error = Nimis;

    /// Create a _Numerus_ from a u8 integer.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let xxvi = Numerus::try_from(26_u8).unwrap();
    /// assert_eq!("XXVI", &xxvi.to_string());
    ///
    /// let cxi: Numerus = 111_u8.try_into().unwrap();
    /// assert_eq!("CXI", &cxi.to_string());
    /// ```
    ///
    /// If the integer is outside the range `1..=3999`, `Err(Nimis)` (_too much_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    ///
    /// let result = Numerus::try_from(0_u8);
    /// assert!(matches!(result, Err(Nimis)));
    /// ```
    fn try_from(vis: u8) -> Result<Self, Self::Error> {
        if vis < 1 {
            Err(Nimis)
        } else {
            Ok(Numerus { vis: vis.into() })
        }
    }
}

impl TryFrom<u16> for Numerus {
    type Error = Nimis;

    /// Create a _Numerus_ from a u16 integer.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let xxvi = Numerus::try_from(26_u16).unwrap();
    /// assert_eq!("XXVI", &xxvi.to_string());
    ///
    /// let cxi: Numerus = 111_u16.try_into().unwrap();
    /// assert_eq!("CXI", &cxi.to_string());
    /// ```
    ///
    /// If the integer is outside the range `1..=3999`, `Err(Nimis)` (_too much_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    ///
    /// let result = Numerus::try_from(0_u16);
    /// assert!(matches!(result, Err(Nimis)));
    ///
    /// let result = Numerus::try_from(4000_u16);
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

impl TryFrom<u32> for Numerus {
    type Error = Nimis;

    /// Create a _Numerus_ from a u32 integer.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let xxvi = Numerus::try_from(26_u32).unwrap();
    /// assert_eq!("XXVI", &xxvi.to_string());
    ///
    /// let cxi: Numerus = 111_u32.try_into().unwrap();
    /// assert_eq!("CXI", &cxi.to_string());
    /// ```
    ///
    /// If the integer is outside the range `1..=3999`, `Err(Nimis)` (_too much_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::convert::TryFrom;
    ///
    /// let result = Numerus::try_from(0_u32);
    /// assert!(matches!(result, Err(Nimis)));
    ///
    /// let result = Numerus::try_from(4000_u32);
    /// assert!(matches!(result, Err(Nimis)));
    /// ```
    fn try_from(vis: u32) -> Result<Self, Self::Error> {
        if (1..=3999).contains(&vis) {
            Ok(Numerus {
                vis: vis.try_into().unwrap(),
            })
        } else {
            Err(Nimis)
        }
    }
}

impl TryFrom<Numerus> for i8 {
    type Error = Nimis;

    /// Get the i8 value of a _Numerus_.
    fn try_from(n: Numerus) -> Result<Self, Self::Error> {
        if n.vis <= (i8::MAX as u16) {
            Ok(n.vis as i8)
        } else {
            Err(Nimis)
        }
    }
}

impl TryFrom<Numerus> for u8 {
    type Error = Nimis;

    /// Get the u8 value of a _Numerus_.
    fn try_from(n: Numerus) -> Result<Self, Self::Error> {
        if n.vis <= (u8::MAX as u16) {
            Ok(n.vis as u8)
        } else {
            Err(Nimis)
        }
    }
}
