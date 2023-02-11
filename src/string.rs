//! API that returns `String`

extern crate alloc;
use alloc::string::String;

use super::{Codec, vec, assert_valid_character_table};

///Encoding function returns string.
///
///Requires feature `alloc`.
///
///# Arguments
///- `src` - Input to encode;
///
///# Panics
///
///In case of required size to be too big or table contains non-ASCII characters
#[inline]
pub fn encode(table: &[u8; 64], src: &[u8]) -> String {
    //User must supply valid table for string conversion to be valid
    debug_assert!(assert_valid_character_table(table));

    let result = vec::encode(table, src);

    unsafe {
        String::from_utf8_unchecked(result)
    }
}

impl<'a> Codec<'a> {
    ///Encoding function returns string.
    ///
    ///Requires feature `alloc`.
    ///
    ///# Arguments
    ///- `src` - Input to encode;
    ///
    ///# Panics
    ///
    ///In case of required size to be too big or table contains non-ASCII characters
    #[inline]
    pub fn encode_into_string(&self, src: &[u8]) -> String {
        let result = vec::encode(self.table, src);

        unsafe {
            String::from_utf8_unchecked(result)
        }
    }
}
