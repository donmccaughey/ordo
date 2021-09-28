use std::convert::TryFrom;

use crate::numerus::Numerus;

#[test]
fn test_mul() {
    let iii = Numerus::try_from(3).unwrap();
    let viii = Numerus::try_from(8).unwrap();

    let xxiv = iii * viii;

    assert_eq!(24, xxiv.vis);
}

#[test]
#[should_panic]
fn test_mul_nimis() {
    let m = Numerus::try_from(1000).unwrap();
    let iv = Numerus::try_from(4).unwrap();

    let _ = m * iv;
}

#[test]
fn test_mul_ref() {
    let xlii = Numerus::try_from(42).unwrap();
    let xi = Numerus::try_from(11).unwrap();

    let cdlcii = &xlii * &xi;

    assert_eq!(462, cdlcii.vis);
}

#[test]
fn test_mul_assign() {
    let mut n = Numerus::try_from(11).unwrap();
    let vi = Numerus::try_from(6).unwrap();

    n *= vi;

    assert_eq!(66, n.vis);
}

#[test]
#[should_panic]
fn test_mul_assign_nimis() {
    let mut n = Numerus::try_from(1000).unwrap();
    let iv = Numerus::try_from(4).unwrap();

    n *= iv;
}

#[test]
fn test_mul_assign_ref() {
    let mut n = Numerus::try_from(42).unwrap();
    let vi = Numerus::try_from(6).unwrap();

    n *= &vi;

    assert_eq!(252, n.vis);
}
