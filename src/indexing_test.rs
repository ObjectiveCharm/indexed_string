use crate::indexing::*;

#[test]
fn test_unicode_index() {
    let test_string = "❤️".to_string();

    let index = test_string.first_index().unwrap();

    assert_eq!(index.utf8_char(), "❤️");

}

#[test]
fn test_unicode_string() {
    let test_string = "❤️é💯".to_string();

    let first_index = test_string.first_index().unwrap();

    let second_index = test_string.index_after(&first_index).unwrap();

    let third_index = test_string.index_after(&second_index).unwrap();


    assert_eq!(first_index.utf8_char(), "❤️");

    assert_eq!(second_index.utf8_char(), "é");

    assert_eq!(third_index.utf8_char(), "💯");
}

#[test]
fn test_null_string() {
    let test_string = "\u{0000}".to_string();

    let first_index = test_string.first_index().unwrap();

    assert_eq!(first_index.utf8_char(), "\u{0000}");
}

