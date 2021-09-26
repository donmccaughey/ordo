mod add;
#[cfg(test)]
mod add_tests;
mod default;
#[cfg(test)]
mod default_tests;
mod display;
#[cfg(test)]
mod display_tests;
mod from;
mod from_str;
#[cfg(test)]
mod from_str_tests;
#[cfg(test)]
mod from_tests;
mod sub;
#[cfg(test)]
mod sub_tests;
#[cfg(test)]
mod test;
mod try_from;
#[cfg(test)]
mod try_from_tests;

/// A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form) Roman numeral in
/// the range `1..=3999` (__I__ to __MMMCMXCIX__).
///
/// _Numerus_ is an integer value type that represents a Roman numeral like __XVII__ or __IX__.
/// _Numeri_ are unsigned integers with a limited range that display as Roman numerals.
///
/// Create _Numeri_ from integers using [`try_from()`](std::convert::TryFrom) or
/// [`try_into()`](std::convert::TryInto).  Get a string representation of a _Numerus_ using
/// [`to_string()`](ToString) or [`format!()`].
/// ```
/// use ordo::Numerus;
/// use std::convert::TryFrom;
/// use std::convert::TryInto;
///
/// let xxvi = Numerus::try_from(26).unwrap();
/// assert_eq!("XXVI", &xxvi.to_string());
///
/// let cxi: Numerus = 111.try_into().unwrap();
/// assert_eq!("CXI", &format!("{}", cxi));
/// ```
///
/// Create _Numeri_ from strings using [`from_str()`](std::str::FromStr) or [`str::parse()`].  Get
/// the integer value of a _Numerus_ using [`from()`](From) or [`into()`](Into).
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xvii = Numerus::from_str("XVII").unwrap();
/// assert_eq!(17, u16::from(xvii));
///
/// let xcix = "XCIX".parse::<Numerus>().unwrap();
/// assert_eq!(99, xcix.into());
/// ```
///
/// _Numeri_ act like unsigned integers, implementing traits [Eq], [Ord], [Hash], [std::ops::Add],
/// [std::ops::AddAssign], [std::ops::Sub] and [std::ops::SubAssign].
/// ```
/// use ordo::Numerus;
/// use std::convert::TryFrom;
///
/// let xxxv = Numerus::try_from(35).unwrap();
/// let xvii = "XVII".parse::<Numerus>().unwrap();
/// let ix = Numerus::try_from(9).unwrap();
///
/// assert!(xxxv > ix);
/// assert!(ix <= xvii);
/// assert_ne!(xxxv, ix);
///
/// let xxvi = xxxv - ix;
/// assert_eq!("XXVI", &xxvi.to_string());
///
/// let xxvi = xvii + ix;
/// assert_eq!("XVII", &xvii.to_string());
/// ```
///
/// Like other Rust integer types, _Numeri_ will panic on overflow in debug builds.
/// ```should_panic
/// use ordo::Numerus;
///
/// /// panics
/// let nimis = Numerus::MAX + Numerus::MIN;
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Numerus {
    vis: u16,
}

impl Numerus {
    /// The smallest _Numerus_ value of __I__ (1).
    pub const MIN: Self = Numerus { vis: 1 };

    /// The largest _Numerus_ value of __MMMCMXCIX__ (3999).
    pub const MAX: Self = Numerus { vis: 3999 };
}
