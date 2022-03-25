
use std::fmt::Debug;
use std::ops::*;


use crate::indexing::*;

/// An wrapper for string added read-only indexing and iterator supports based on Vec
/// To modify string, just modify the struct and generate new string using `IndexedString::string(&self) -> String`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndexedString {
    /// vec of every utf-8 char, a char may take 1 to 6 bytes with 1 - 4 byte(s) utf-8 code point and optional 3 bytes variation selector which is related to display
    pub utf8_char_vec: Vec<Box<String>>
}


/// Generate string from self
impl IndexedString {
    fn string(&self) -> String {
        self.to_string()
    }
}

/// String representation
impl ToString for IndexedString {
    fn to_string(&self) -> String {
        let mut string = String::new();

        for item in &self.utf8_char_vec {
            string.push_str(item.as_ref());
        }
        string
    }
}

/// Convert to String and taking ownership
impl Into<String> for IndexedString {
    fn into(self) -> String {
        self.to_string()
    }
}

/// Create `IndexedString` from String, takes O(n) for method needs traverse the string to build index
impl From<String> for IndexedString {
    fn from(string: String) -> Self {
        let mut utf8_char_vec: Vec<Box<String>> = Vec::new();

        let mut index = string.first_index();

        loop {
            if let Some(current_index) = index {
                let utf8_char = string.utf8_char_at(&current_index).to_string();
                utf8_char_vec.push(Box::from(utf8_char));
                index = string.index_after(&current_index);
            } else {
                break;
            }
        }

        Self {
            utf8_char_vec
        }
    }
}

/// Create `IndexedString` from String slice
impl From<&str> for IndexedString {
    fn from(str: &str) -> Self {
        Self::from(str.to_string())
    }
}

/// Accessing `IndexString` immutably through underlying vec
impl Index<usize> for IndexedString {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.utf8_char_vec[index].as_ref()
    }
}

impl IndexMut<usize> for IndexedString {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.utf8_char_vec[index].as_mut()
    }
}

/// Make immutable method of vec available
impl Deref for IndexedString {
    type Target = Vec<Box<String>>;

    fn deref(&self) -> &Self::Target {
        &self.utf8_char_vec
    }
}

impl DerefMut for IndexedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.utf8_char_vec
    }
}

