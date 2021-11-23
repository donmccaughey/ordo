use crate::errors::Irritus;
use crate::iter::char::fallible::{CharSequences, Iterators};

#[test]
fn test_canonical_chars() {
    let mater = "m\u{0101}ter";
    let result = mater
        .fallible_chars()
        .canonical_chars()
        .collect::<Result<String, Irritus>>();
    assert_eq!("m\u{0101}ter", result.unwrap());

    let mater = "mater!";
    let result = mater
        .fallible_chars()
        .canonical_chars()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
