use crate::locutio::Locutio;
use crate::Orthographia;

#[test]
fn test_display_one_word() {
    let vacca = Orthographia::try_from_ascii("vacca").unwrap();
    let locutio = Locutio::one_word(vacca);
    assert_eq!("vacca", format!("{}", locutio));
}

#[test]
fn test_display_two_words() {
    let viginti = Orthographia::try_from_ascii("vi'ginti'").unwrap();
    let duo = Orthographia::try_from_ascii("duo").unwrap();
    let locutio = Locutio::two_words(viginti, duo);
    assert_eq!("vi'ginti' duo", format!("{}", locutio));
}
