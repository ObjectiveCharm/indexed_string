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
    let str = "CafΓ© du π";

    let indexed_string = IndexedString::from(str);

    assert_eq!(indexed_string.to_string(), str);
}

#[test]
fn access_indexed_string() {
    let str = "CafΓ© du π";

    let indexed_string = IndexedString::from(str);

    println!("{}", indexed_string[8]);

    assert_eq!(indexed_string[8], "π");
}

#[test]
fn mutating_index_string() {
    let str = "πβ€οΈΓ©π―πΊπ¦";

    let mut indexed_string = IndexedString::from(str);

    indexed_string[2] = "2".to_string();


    assert_eq!(indexed_string.to_string(), "πβ€οΈ2π―πΊπ¦")
}

#[test]
fn iterator_over_indexed_string() {
    let str = "πβ€οΈΓ©π―";

    let indexed_string = IndexedString::from(str);

    for (index, item) in indexed_string.iter().enumerate() {
        match index {
            0 => {
                assert_eq!(item.deref(), "π")
            }
            1 => {
                assert_eq!(item.deref(), "β€οΈ")
            }
            2 => {
                assert_eq!(item.deref(), "Γ©")
            }
            3 => {
                assert_eq!(item.deref(), "π―")
            }
            _ => {
                panic!();
            }
        }
    }


}
