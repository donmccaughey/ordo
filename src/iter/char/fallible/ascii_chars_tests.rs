use crate::errors::Irritus;
use crate::iter::char::fallible::{Filter, Iterators};

#[test]
fn test_ascii_chars() {
    let mater = "m\u{0101}ter";
    let result = mater
        .filter()
        .ascii_chars()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let mater = "ma'ter";
    let result = mater
        .filter()
        .ascii_chars()
        .collect::<Result<String, Irritus>>();
    assert_eq!("ma'ter", result.unwrap());
}
