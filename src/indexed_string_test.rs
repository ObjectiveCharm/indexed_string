use std::ops::Deref;
use crate::indexed_string::*;

#[test]
fn create_indexed_string() {
    let str = "aaabc";

    let indexed_string = IndexedString::from(str);

    assert_eq!(indexed_string.to_string(), str.to_string());
}

#[test]
fn create_indexed_string_with_unicode() {
    let str = "Café du 🌍";

    let indexed_string = IndexedString::from(str);

    assert_eq!(indexed_string.to_string(), str);
}

#[test]
fn access_indexed_string() {
    let str = "Café du 🌍";

    let indexed_string = IndexedString::from(str);

    println!("{}", indexed_string[8]);

    assert_eq!(indexed_string[8], "🌍");
}

#[test]
fn mutating_index_string() {
    let str = "🌍❤️é💯🇺🇦";

    let mut indexed_string = IndexedString::from(str);

    indexed_string[2] = "2".to_string();


    assert_eq!(indexed_string.to_string(), "🌍❤️2💯🇺🇦")
}

#[test]
fn iterator_over_indexed_string() {
    let str = "🌍❤️é💯";

    let indexed_string = IndexedString::from(str);

    for (index, item) in indexed_string.iter().enumerate() {
        match index {
            0 => {
                assert_eq!(item.deref(), "🌍")
            }
            1 => {
                assert_eq!(item.deref(), "❤️")
            }
            2 => {
                assert_eq!(item.deref(), "é")
            }
            3 => {
                assert_eq!(item.deref(), "💯")
            }
            _ => {
                panic!();
            }
        }
    }


}
