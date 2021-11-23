use crate::errors::Irritus;
use crate::iter::char::fallible::{CharSequences, Iterators};

#[test]
fn test_long_vowel_macrons() {
    let mater = "m\u{0101}ter";
    let result = mater
        .fallible_chars()
        .long_vowel_macrons()
        .collect::<Result<String, Irritus>>();
    assert_eq!("m\u{0101}ter", result.unwrap());

    let mater = "ma\u{0304}ter";
    let result = mater
        .fallible_chars()
        .long_vowel_macrons()
        .collect::<Result<String, Irritus>>();
    assert_eq!("m\u{0101}ter", result.unwrap());

    let mater = "m\u{0304}ater";
    let result = mater
        .fallible_chars()
        .long_vowel_macrons()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let mater = "ma\u{0304}\u{0304}ter";
    let result = mater
        .fallible_chars()
        .long_vowel_macrons()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let mater = "m\u{0101}\u{0304}ter";
    let result = mater
        .fallible_chars()
        .long_vowel_macrons()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
