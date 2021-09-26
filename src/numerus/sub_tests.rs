use std::convert::TryFrom;

use crate::numerus::Numerus;

#[test]
fn test_sub() {
    let viii = Numerus::try_from(8).unwrap();
    let iii = Numerus::try_from(3).unwrap();

    let v = viii - iii;

    assert_eq!(5, v.vis);
}

#[test]
#[should_panic]
fn test_sub_nimis_nihil() {
    let a = Numerus::try_from(7).unwrap();
    let b = Numerus::try_from(7).unwrap();

    let _ = a - b;
}

#[test]
#[should_panic]
fn test_sub_nimis() {
    let a = Numerus::try_from(7).unwrap();
    let b = Numerus::try_from(8).unwrap();

    let _ = a - b;
}

#[test]
fn test_sub_ref() {
    let xlii = Numerus::try_from(42).unwrap();
    let xi = Numerus::try_from(11).unwrap();

    let xxxi = &xlii - &xi;

    assert_eq!(31, xxxi.vis);
}

#[test]
fn test_sub_assign() {
    let mut n = Numerus::try_from(11).unwrap();
    let vi = Numerus::try_from(6).unwrap();

    n -= vi;

    assert_eq!(5, n.vis);
}

#[test]
#[should_panic]
fn test_sub_assign_nimis_nihil() {
    let mut n = Numerus::try_from(10).unwrap();
    let x = Numerus::try_from(10).unwrap();

    n -= x;
}

#[test]
#[should_panic]
fn test_sub_assign_nimis() {
    let mut n = Numerus::try_from(10).unwrap();
    let ix = Numerus::try_from(11).unwrap();

    n -= ix;
}

#[test]
fn test_sub_assign_ref() {
    let mut n = Numerus::try_from(42).unwrap();
    let vi = Numerus::try_from(6).unwrap();

    n -= &vi;

    assert_eq!(36, n.vis);
}
