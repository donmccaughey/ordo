use crate::errors::Irritus;
use crate::iter::char::fallible::{Filter, Iterators};

#[test]
fn test_solo_hyphens() {
    let absunt = "ab-sunt";
    let result = absunt
        .filter()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert_eq!("ab-sunt", result.unwrap());

    let absunt = "ab--sunt";
    let result = absunt
        .filter()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let que = "-que";
    let result = que
        .filter()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert_eq!("-que", result.unwrap());

    let que = "--que";
    let result = que
        .filter()
        .solo_hyphens()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
