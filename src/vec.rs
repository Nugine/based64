//! High level functions returning vector as convenience

extern crate alloc;
use alloc::vec::Vec;

use super::*;

///Encoding function returns vector with data written.
///
///Requires feature `alloc`.
///
///`src` - Input to encode;
///
///Returns `Some` if successful, containing number of bytes written.
///Returns `None` if data cannot be encoded due to insufficient buffer size or size calculation overflow happens.
#[inline]
pub fn encode(table: &[u8; 64], src: &[u8]) -> Option<Vec<u8>> {
    let mut required_len = encode_len(src.len());
    let mut result = Vec::with_capacity(required_len);
    unsafe {
        let ptr = core::ptr::NonNull::new_unchecked(result.as_mut_ptr());
        match raw::encode(table, src, ptr, &mut required_len) {
            true => {
                result.set_len(required_len);
            },
            false => return None,
        }
    }

    Some(result)
}

///Decoding function returns vector with data written.
///
///Requires feature `alloc`.
///
///`src` - Input to decode;
///
///Returns `Some` if successful, containing number of bytes written.
///Returns `None` if data cannot be encoded due to insufficient buffer size.
#[inline]
pub fn decode(table: &[u8; 64], src: &[u8]) -> Option<Vec<u8>> {
    let mut required_len = decode_len(src);
    let mut result = Vec::with_capacity(required_len);
    unsafe {
        let ptr = core::ptr::NonNull::new_unchecked(result.as_mut_ptr());
        match raw::decode(table, src, ptr, &mut required_len) {
            true => {
                result.set_len(required_len);
            },
            false => return None,
        }
    }

    Some(result)
}
