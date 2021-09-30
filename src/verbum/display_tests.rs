use crate::verbum::Verbum;

#[test]
fn test_to_string() {
    let v = Verbum::try_from_ascii("hodie'").unwrap();
    assert_eq!("hodie\u{0304}", v.to_string());
}
