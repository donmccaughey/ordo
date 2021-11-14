use crate::iter::char::Iterators;

#[test]
fn test_no_macrons() {
    let mater = "m\u{0101}ter";
    let no_macrons = mater.chars().no_macrons().collect::<String>();
    assert_eq!("mater", no_macrons);

    let canis = "canis";
    let unchanged = canis.chars().no_macrons().collect::<String>();
    assert_eq!("canis", unchanged);
}
