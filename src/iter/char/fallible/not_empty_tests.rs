use crate::errors::Irritus;
use crate::iter::char::fallible::{Filter, Iterators};

#[test]
fn test_not_empty() {
    let vacca = "vacca";
    let result = vacca
        .filter()
        .not_empty()
        .collect::<Result<String, Irritus>>();
    assert_eq!("vacca", result.unwrap());

    let empty = "";
    let result = empty
        .filter()
        .not_empty()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
