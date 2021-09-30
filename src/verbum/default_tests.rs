use crate::verbum::Verbum;

#[test]
fn test_default() {
    let v = Verbum::default();
    assert_eq!("nu\u{0304}llus", &v.s);
}
