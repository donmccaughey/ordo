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

    let esus = "\u{0112}sus"; // "Ēsus"
    let result = esus
        .fallible_chars()
        .initial_caps()
        .collect::<Result<String, Irritus>>();
    assert_eq!("\u{0112}sus", result.unwrap());

    let exire = "ex\u{012a}re"; // exĪre
    let result = exire
        .fallible_chars()
        .initial_caps()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let xvii = "XVII";
    let result = xvii
        .fallible_chars()
        .initial_caps()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
