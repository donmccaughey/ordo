use crate::errors::Nimis;
use crate::numerus::Numerus;
use std::convert::{TryFrom, TryInto};

#[test]
fn test_numerus_try_from_unqualified_integer_literal() {
    let result = Numerus::try_from(0);
    assert!(matches!(result, Err(Nimis)));

    let result = Numerus::try_from(1);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(3999);
    assert!(result.is_ok());
    assert_eq!(3999, result.unwrap().vis);

    let result = Numerus::try_from(4000);
    assert!(matches!(result, Err(Nimis)));
}

#[test]
fn test_numerus_try_from_i8() {
    let result = Numerus::try_from(0_i8);
    assert!(matches!(result, Err(Nimis)));

    let result = Numerus::try_from(1_i8);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(i8::MAX);
    assert!(result.is_ok());
    assert_eq!(127, result.unwrap().vis);
}

#[test]
fn test_numerus_try_from_i16() {
    let result = Numerus::try_from(0_i16);
    assert!(matches!(result, Err(Nimis)));

    let result = Numerus::try_from(1_i16);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(3999_i16);
    assert!(result.is_ok());
    assert_eq!(3999, result.unwrap().vis);

    let result = Numerus::try_from(4000_i16);
    assert!(matches!(result, Err(Nimis)));
}

#[test]
fn test_numerus_try_from_i32() {
    let result = Numerus::try_from(0_i32);
    assert!(matches!(result, Err(Nimis)));

    let result = Numerus::try_from(1_i32);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(3999_i32);
    assert!(result.is_ok());
    assert_eq!(3999, result.unwrap().vis);

    let result = Numerus::try_from(4000_i32);
    assert!(matches!(result, Err(Nimis)));
}

#[test]
fn test_numerus_try_from_u8() {
    let result = Numerus::try_from(0_u8);
    assert!(matches!(result, Err(Nimis)));

    let result = Numerus::try_from(1_u8);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(u8::MAX);
    assert!(result.is_ok());
    assert_eq!(255, result.unwrap().vis);
}

#[test]
fn test_numerus_try_from_u16() {
    let result = Numerus::try_from(0_u16);
    assert!(matches!(result, Err(Nimis)));

    let result = Numerus::try_from(1_u16);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(3999_u16);
    assert!(result.is_ok());
    assert_eq!(3999, result.unwrap().vis);

    let result = Numerus::try_from(4000_u16);
    assert!(matches!(result, Err(Nimis)));
}

#[test]
fn test_numerus_try_from_u32() {
    let result = Numerus::try_from(0_u32);
    assert!(matches!(result, Err(Nimis)));

    let result = Numerus::try_from(1_u32);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().vis);

    let result = Numerus::try_from(3999_u32);
    assert!(result.is_ok());
    assert_eq!(3999, result.unwrap().vis);

    let result = Numerus::try_from(4000_u32);
    assert!(matches!(result, Err(Nimis)));
}

#[test]
fn test_i8_try_from_numerus() {
    let n: Numerus = 127.try_into().unwrap();
    let result = i8::try_from(n);
    assert!(result.is_ok());
    assert_eq!(i8::MAX, result.unwrap());

    let n: Numerus = 128.try_into().unwrap();
    let result = i8::try_from(n);
    assert!(matches!(result, Err(Nimis)));
}

#[test]
fn test_u8_try_from_numerus() {
    let n: Numerus = 255.try_into().unwrap();
    let result = u8::try_from(n);
    assert!(result.is_ok());
    assert_eq!(u8::MAX, result.unwrap());

    let n: Numerus = 256.try_into().unwrap();
    let result = u8::try_from(n);
    assert!(matches!(result, Err(Nimis)));
}
