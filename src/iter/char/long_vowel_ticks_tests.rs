use crate::iter::char::Iterators;

#[test]
fn test_long_vowel_ticks() {
    let mater = "m\u{0101}ter";
    let ascii = mater.chars().long_vowel_ticks().collect::<String>();
    assert_eq!("ma'ter", ascii);

    let canis = "canis";
    let ascii = canis.chars().long_vowel_ticks().collect::<String>();
    assert_eq!("canis", ascii);
}
