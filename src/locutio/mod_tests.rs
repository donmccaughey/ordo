use crate::locutio::Locutio;
use crate::Orthographia;

#[test]
fn test_one_word() {
    let canis = Orthographia::try_from_ascii("canis").unwrap();
    let locutio = Locutio::one_word(canis);
    assert_eq!(1, locutio.orthographiae.len());
    assert_eq!("canis", &locutio.orthographiae[0].to_string());
}

#[test]
fn test_two_words() {
    let amatus = Orthographia::try_from_ascii("amatus").unwrap();
    let es = Orthographia::try_from_ascii("es").unwrap();
    let locutio = Locutio::two_words(amatus, es);
    assert_eq!(2, locutio.orthographiae.len());
    assert_eq!("amatus", &locutio.orthographiae[0].to_string());
    assert_eq!("es", &locutio.orthographiae[1].to_string());
}

#[test]
fn test_try_new() {
    let amatus = Orthographia::try_from_ascii("amatus").unwrap();
    let es = Orthographia::try_from_ascii("es").unwrap();
    let result = Locutio::try_new(vec![amatus, es]);
    assert!(result.is_ok());
    let locutio = result.unwrap();
    assert_eq!(2, locutio.orthographiae.len());
    assert_eq!("amatus", &locutio.orthographiae[0].to_string());
    assert_eq!("es", &locutio.orthographiae[1].to_string());
}

#[test]
fn test_try_new_irritus() {
    let result = Locutio::try_new(vec![]);
    assert!(matches!(result, Err(Irritus)));
}
