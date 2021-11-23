use crate::errors::Irritus;
use crate::iter::char::fallible::{CharSequences, Iterators};

#[test]
fn test_solo_hyphens() {
    let absunt = "ab-sunt";
    let result = absunt
        .fallible_chars()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert_eq!("ab-sunt", result.unwrap());

    let absunt = "ab--sunt";
    let result = absunt
        .fallible_chars()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let que = "-que";
    let result = que
        .fallible_chars()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert_eq!("-que", result.unwrap());

    let que = "--que";
    let result = que
        .fallible_chars()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
