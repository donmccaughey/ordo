use std::collections::HashMap;
use crate::locutio::Locutio;
use crate::Orthographia;

#[test]
fn test_clone() {
    let nauta = Orthographia::try_from_ascii("nauta").unwrap();
    let locutio1 = Locutio::one_word(nauta);
    let locutio2 = locutio1.clone();
    assert_eq!(locutio1.orthographiae, locutio2.orthographiae);
}

#[test]
fn test_eq() {
    let cauda = Orthographia::try_from_ascii("cauda").unwrap();
    let locutio1 = Locutio::one_word(cauda);
    let locutio2 = locutio1.clone();
    assert_eq!(locutio1, locutio2);
}

#[test]
fn test_ord() {
    let amatus = Orthographia::try_from_ascii("amatus").unwrap();
    let es = Orthographia::try_from_ascii("es").unwrap();
    let est = Orthographia::try_from_ascii("est").unwrap();
    let amatus_es = Locutio::two_words(amatus.clone(), es);
    let amatus_est = Locutio::two_words(amatus, est);
    assert!(amatus_es < amatus_est);
    assert!(amatus_est > amatus_es);
}

#[test]
fn test_hash() {
    let viginti = Orthographia::try_from_ascii("vi'ginti'").unwrap();
    let duo = Orthographia::try_from_ascii("duo").unwrap();
    let quattuor = Orthographia::try_from_ascii("quattuor").unwrap();
    let xxii = Locutio::two_words(viginti.clone(), duo);
    let xxiv = Locutio::two_words(viginti, quattuor);

    let mut dict = HashMap::new();
    dict.insert(xxii, 22);
    dict.insert(xxiv, 24);
    assert_eq!(dict.len(), 2);
}

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
