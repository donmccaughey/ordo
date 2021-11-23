use crate::errors::Irritus;
use crate::iter::char::fallible::{CharSequences, Iterators};

#[test]
fn test_initial_caps() {
    let vacca = "vacca";
    let result = vacca
        .fallible_chars()
        .initial_caps()
        .collect::<Result<String, Irritus>>();
    assert_eq!("vacca", result.unwrap());

    let marcus = "Marcus";
    let result = marcus
        .fallible_chars()
        .initial_caps()
        .collect::<Result<String, Irritus>>();
    assert_eq!("Marcus", result.unwrap());

    let marcus = "XVII";
    let result = marcus
        .fallible_chars()
        .initial_caps()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
