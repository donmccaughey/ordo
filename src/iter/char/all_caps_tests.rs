use crate::iter::char::Iterators;

#[test]
fn test_all_caps() {
    let mater = "m\u{0101}ter";
    let caps = mater.chars().all_caps().collect::<String>();
    assert_eq!("M\u{0100}TER", caps);

    let canis = "canis";
    let caps = canis.chars().all_caps().collect::<String>();
    assert_eq!("CANIS", caps);
}
