
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::range::*;


/// Underlying data structure for indexing string
/// for more about utf8 character, please refer to [the wiki page](https://en.wikipedia.org/wiki/UTF-8)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct StringIndex {
    // location in unicode view
    pub(crate) value: usize,

    // start code point in raw byte vec
    pub(crate) raw_start_index: usize,

    // length of the utf8 char
    pub(crate) raw_len: u8,

    // raw bytes(unicode scalar view) of utf8 char
    pub(crate) bytes: [u8; 4],

    // optional extra variation selector
    pub(crate) extra_variation_selector: Option<[u8; 3]>
}

impl StringIndex {

    pub fn zero() -> Self {
        Self {
            value: 0,
            raw_start_index: 0,
            raw_len: 0,
            bytes: [0, 0, 0, 0],
            extra_variation_selector: None
        }
    }

    pub fn raw_bytes_description(&self) -> String {
        format!("{:?}", self.bytes)
    }

    pub fn variation_selector_description(&self) -> String {
        if let Some(bytes) = self.extra_variation_selector {
            format!("{:?}", bytes)
        } else {
            format!("Empty")
        }
    }

    /// The corresponding utf8 char
    pub fn utf8_char(&self) -> String {
        if self.bytes == [0, 0, 0, 0] {
            // to deal with \u{0000} (NULL) value
            "\u{0}".to_string()
        } else {
            let mut bytes: Vec<u8> = Vec::new();
            self.bytes.iter().for_each(|&i| {
                // filter all padding 0 for character has less than 4 bytes
                if i != 0 { bytes.push(i) }
            });
            if let Some(variation_selector) = self.extra_variation_selector {
                variation_selector.iter().for_each(|&i| {
                    bytes.push(i)
                });
            }
            String::from_utf8(bytes).unwrap()
        }
    }
}

impl Display for StringIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "index: {},\
         bytes: [{}],\
         location in bytes: {}\
         bytes length: {}\
         variation selector: {}",
               self.value,
               self.raw_bytes_description(),
               self.raw_start_index,
               self.raw_len,
               self.variation_selector_description())
    }
}



#[derive(Debug)]
/// Indexing errors
pub enum IndexingError {
    /// `UnexpectedTerminate` is thrown when read broken string, location is last raw index before bad char
    UnexpectedTerminate {
        location: usize
    },

    /// `InvalidUnicodePoint` is thrown when invalid Unicode found
    InvalidUnicodePoint {
        location: usize,
        code_point: usize
    }
}


impl IndexingError {
    fn error_message(&self) -> String {
        match self {
            IndexingError::UnexpectedTerminate { location } =>
                format!("unexpected terminated at location {}", location.to_string()),
            IndexingError::InvalidUnicodePoint { location, code_point} =>
                format!("Invalid Unicode Scalar {} found at {}", code_point, location)
        }
    }

}

impl Display for IndexingError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error_message())?;
        Ok(())
    }
}

impl Error for IndexingError { }

/// Indexing implementation
pub trait Indexing {
    /// Get index after parameter in the string
    /// Won't panic but throw error when access broken string, for debug usage
    /// Using `String::from_utf8_lossy` to prevent reading broken string
    fn try_index_after(&self, index: &StringIndex) -> Result<Option<StringIndex>, IndexingError>;

    /// Get utf-8 char in specified index
    fn utf8_char_at(&self, index: &StringIndex) -> &str;

    /// Unchecked and will panic when access broken string
    fn index_after(&self, index: &StringIndex) -> Option<StringIndex> {
        match self.try_index_after(index) {
            Ok(index) => index,
            Err(error) => {
                match error {
                    IndexingError::UnexpectedTerminate { location } => {
                        panic!("unexpected terminate at {}", location)
                    }
                    IndexingError::InvalidUnicodePoint {location, code_point} => {
                        panic!("invalid utf-8 codepoint {} found at {}", code_point, location)
                    }
                }
            }
        }
    }

    /// First index of string
    fn first_index(&self) -> Option<StringIndex> {
        self.index_after(&StringIndex::zero())
    }

}


impl Indexing for String {

    // Implementation
    fn try_index_after(&self, index: &StringIndex) -> Result<Option<StringIndex>, IndexingError> {
        let raw_start_index = index.raw_start_index + index.raw_len as usize;
        let value = index.value + 1;
        let raw_bytes = self.as_bytes();

        // raw_len of StringIndex equals to `character_len` plus optional `extra_variation_selector` len which always equals to 3
        // initial assigned value is just placeholder
        let mut character_len: u8;
        let mut extra_variation_selector: Option<[u8; 3]>;
        let mut raw_len: u8;

        let replacing_bytes = |location: usize, n: u8, dst: &mut [u8], raw_bytes: &[u8]| {
            if let Some(src_bytes) = raw_bytes.items(location + 1, n as usize) {
                dst.replace_items(1, src_bytes);
                Ok(())
            } else {
                Err(IndexingError::UnexpectedTerminate { location })
            }
        };

        if let Some(first_byte) = raw_bytes.get(raw_start_index) {
            // to store raw bytes of each utf8 character
            let mut bytes: [u8; 4] = [*first_byte, 0, 0 ,0];
                if *first_byte >> 7 == 0 {
                    // When first code point begin with 0, the utf-8 character has 1 byte
                    character_len = 1;

                } else if *first_byte >> 3 == 0b00011110u8 {
                    // When first code point begin with 11110, the utf-8 character has 4 bytes
                    character_len = 4;
                    replacing_bytes(raw_start_index, 3, &mut bytes, raw_bytes)?
                } else if *first_byte >> 4 == 0b00001110u8 {
                    // When first code point begin with 1110, the utf-8 character has 3 bytes
                    character_len = 3;

                    replacing_bytes(raw_start_index, 2, &mut bytes, raw_bytes)?
                } else if *first_byte >> 5 == 0b00000110u8 {
                    // When first code point begin with 110, the utf-8 character has 2 bytes
                    character_len = 2;

                    replacing_bytes(raw_start_index, 1, &mut bytes, raw_bytes)?
                } else {
                    return Err(IndexingError::InvalidUnicodePoint { location: index.value + 1,
                            code_point: *first_byte as usize
                        })
                }

            // try to detect variation selector witch between \u{fe00} and \u{fe0f}
            // for more about variation selector please refer to [the wiki page](https://en.wikipedia.org/wiki/Variation_Selectors_(Unicode_block))
            let detect_variation_selector = |raw_bytes: &[u8], location: usize| {
                if location + 3 <= raw_bytes.len() {
                    if raw_bytes[location] == 0xef && raw_bytes[location + 1] == 0xb8 {
                        Some([raw_bytes[location], raw_bytes[location + 1], raw_bytes[location + 2]])
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            extra_variation_selector = detect_variation_selector(raw_bytes, raw_start_index + character_len as usize);

            if extra_variation_selector != None {
                raw_len = character_len + 3;
            } else {
                raw_len = character_len;
            }

            Ok(Some(StringIndex {
                value,
                raw_start_index,
                raw_len,
                bytes,
                extra_variation_selector
            }))
        } else {
            Ok(None)
        }


    }

    fn utf8_char_at(&self, index: &StringIndex) -> &str {
        let raw_end_index = index.raw_start_index + index.raw_len as usize;
        &self[index.raw_start_index..raw_end_index]
    }
}

