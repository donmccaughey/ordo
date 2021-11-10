use crate::Numerus;

use std::convert::TryInto;

/// Convert a _Numerus_ into an [i16] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = i16::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: i16 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for i16 {
    fn from(n: Numerus) -> Self {
        debug_assert!(Numerus::MAX.vis < i16::MAX as u16);
        n.vis.try_into().unwrap()
    }
}

/// Convert a _Numerus_ into an [i32] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = i32::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: i32 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for i32 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

/// Convert a _Numerus_ into an [i64] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = i64::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: i64 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for i64 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

/// Convert a _Numerus_ into an [i64] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = i128::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: i128 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for i128 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

/// Convert a _Numerus_ into an [isize] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = isize::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: isize = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for isize {
    fn from(n: Numerus) -> Self {
        debug_assert!((Numerus::MAX.vis as usize) < (isize::MAX as usize));
        n.vis.try_into().unwrap()
    }
}

/// Convert a _Numerus_ into a [u16] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = u16::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: u16 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for u16 {
    fn from(n: Numerus) -> Self {
        n.vis
    }
}

/// Convert a _Numerus_ into a [u32] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = u32::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: u32 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for u32 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

/// Convert a _Numerus_ into a [u64] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = u64::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: u64 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for u64 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

/// Convert a _Numerus_ into a [u128] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = u128::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: u128 = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for u128 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

/// Convert a _Numerus_ into a [usize] integer.
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xlii = Numerus::from_str("XLII").unwrap();
///
/// let a = usize::from(xlii);
/// assert_eq!(a, 42);
///
/// let b: usize = xlii.into();
/// assert_eq!(b, 42);
/// ```
impl From<Numerus> for usize {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}
