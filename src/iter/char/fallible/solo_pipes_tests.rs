use crate::errors::Irritus;
use crate::iter::char::fallible::{Filter, Iterators};

#[test]
fn test_solo_pipes() {
    let canit = "can|it";
    let result = canit
        .filter()
        .solo_pipes()
        .collect::<Result<String, Irritus>>();
    assert_eq!("can|it", result.unwrap());

    let canit = "can||it";
    let result = canit
        .filter()
        .solo_pipes()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let us = "|us";
    let result = us
        .filter()
        .solo_pipes()
        .collect::<Result<String, Irritus>>();
    assert_eq!("|us", result.unwrap());

    let us = "||us";
    let result = us
        .filter()
        .solo_pipes()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));

    let stat = "stat|";
    let result = stat
        .filter()
        .solo_pipes()
        .collect::<Result<String, Irritus>>();
    assert_eq!("stat|", result.unwrap());

    let stat = "stat||";
    let result = stat
        .filter()
        .solo_pipes()
        .collect::<Result<String, Irritus>>();
    assert!(matches!(result, Err(Irritus)));
}
