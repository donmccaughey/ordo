use crate::vocabulum::Orthographia;

#[test]
fn test_to_string() {
    let forma = Orthographia::try_from_ascii("hodie'").unwrap();
    assert_eq!("hodi\u{0113}", forma.to_string());
}
