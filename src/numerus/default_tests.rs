use crate::numerus::Numerus;

#[test]
fn test_default() {
    let n = Numerus::default();
    assert_eq!("I", &n.to_string());
}
