use crate::locutio::Locutio;
use crate::Orthographia;

#[test]
fn test_from_word() {
    let canis = Orthographia::try_from_ascii("canis").unwrap();
    let locutio = Locutio::from_word(canis);
    assert_eq!(1, locutio.orthographiae.len());
    assert_eq!("canis", &locutio.orthographiae[0].to_string());
}

#[test]
fn test_new() {
    let amatus = Orthographia::try_from_ascii("amatus").unwrap();
    let es = Orthographia::try_from_ascii("es").unwrap();
    let locutio = Locutio::new(vec![amatus, es]);
    assert_eq!(2, locutio.orthographiae.len());
    assert_eq!("amatus", &locutio.orthographiae[0].to_string());
    assert_eq!("es", &locutio.orthographiae[1].to_string());
}
