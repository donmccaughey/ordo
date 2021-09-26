use crate::numerus::Numerus;
use std::convert::TryFrom;

#[test]
fn test_try_from() {
    let result = Numerus::try_from(0);
    assert!(result.is_err());

    let result = Numerus::try_from(1);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(3999);
    assert!(result.is_ok());
    assert_eq!(3999, result.unwrap().vis);

    let result = Numerus::try_from(4000);
    assert!(result.is_err());
}

#[test]
fn test_try_from_i8() {
    let n = Numerus::try_from(127).unwrap();
    let result = i8::try_from(n);
    assert!(result.is_ok());
    assert_eq!(i8::MAX, result.unwrap());

    let n = Numerus::try_from(128).unwrap();
    let result = i8::try_from(n);
    assert!(result.is_err());
}

#[test]
fn test_try_from_u8() {
    let n = Numerus::try_from(255).unwrap();
    let result = u8::try_from(n);
    assert!(result.is_ok());
    assert_eq!(u8::MAX, result.unwrap());

    let n = Numerus::try_from(256).unwrap();
    let result = u8::try_from(n);
    assert!(result.is_err());
}
