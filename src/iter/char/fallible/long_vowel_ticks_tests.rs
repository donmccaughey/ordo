use crate::errors::Irritus;
use crate::iter::char::fallible::{Filter, Iterators};

#[test]
fn test_long_vowel_ticks() {
    let mater = "ma'ter";
    let result = mater
        .filter()
        .long_vowel_ticks()
        .collect::<Result<String, Irritus>>();
    assert_eq!("m\u{0101}ter", result.unwrap());

    let mater = "m'ater";
    let result = mater
        .filter()
        .long_vowel_ticks()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let mater = "ma''ter";
    let result = mater
        .filter()
        .long_vowel_ticks()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
