extern crate alloc;
use alloc::string::String;

use super::{vec, assert_valid_character_table};

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