use std::convert::TryFrom;

use crate::numerus::Numerus;

#[test]
fn test_i16_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let i = i16::from(xlii);
    assert_eq!(42_i16, i);
}

#[test]
fn test_i32_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let i = i32::from(xlii);
    assert_eq!(42_i32, i);
}

#[test]
fn test_i64_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let i = i64::from(xlii);
    assert_eq!(42_i64, i);
}

#[test]
fn test_i128_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let i = i128::from(xlii);
    assert_eq!(42_i128, i);
}

#[test]
fn test_isize_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let i = isize::from(xlii);
    assert_eq!(42_isize, i);
}

#[test]
fn test_u16_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let u = u16::from(xlii);
    assert_eq!(42_u16, u);
}

#[test]
fn test_u32_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let u = u32::from(xlii);
    assert_eq!(42_u32, u);
}

#[test]
fn test_u64_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let u = u64::from(xlii);
    assert_eq!(42_u64, u);
}

#[test]
fn test_u128_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let u = u128::from(xlii);
    assert_eq!(42_u128, u);
}

#[test]
fn test_usize_from() {
    let xlii = Numerus::try_from(42).unwrap();
    let u = usize::from(xlii);
    assert_eq!(42_usize, u);
}
