//! High level functions returning vector as convenience

extern crate alloc;
use alloc::vec::Vec;

use super::{Codec, encode_len, decode_len, raw};

///Encoding function returns vector with data written.
///
///Requires feature `alloc`.
///
///# Arguments
///- `src` - Input to encode;
///
///# Panics
///
///In case of required size to be too big
#[inline]
pub fn encode(table: &[u8; 64], src: &[u8]) -> Vec<u8> {
    let mut required_len = encode_len(src.len());
    //Make sure that we don't overflow (which is unlikely but still)
    //>= for case of zero sized input
    assert!(required_len >= src.len());

    let mut result = Vec::with_capacity(required_len);
    unsafe {
        let ptr = core::ptr::NonNull::new_unchecked(result.as_mut_ptr());
        raw::encode_inner(table, src, ptr, &mut required_len);
        result.set_len(required_len);
    }

    result
}

///Decoding function returns vector with data written.
///
///Requires feature `alloc`.
///
///# Arguments
///
///- `src` - Input to decode;
///
///# Result
///
///Returns `Some` if successful, containing decoded output
///
///Returns `None` if `src` is invalid input.
#[inline]
pub fn decode(table: &[u8; 64], src: &[u8]) -> Option<Vec<u8>> {
    let mut required_len = decode_len(src);
    let mut result = Vec::with_capacity(required_len);
    unsafe {
        let ptr = core::ptr::NonNull::new_unchecked(result.as_mut_ptr());
        match raw::decode_inner(table, src, ptr, &mut required_len) {
            true => {
                result.set_len(required_len);
            },
            false => return None,
        }
    }

    Some(result)
}

impl<'a> Codec<'a> {
    ///Encoding function returns vector with data written.
    ///
    ///Requires feature `alloc`.
    ///
    ///# Arguments
    ///- `src` - Input to encode;
    ///
    ///# Panics
    ///
    ///In case of required size to be too big
    #[inline(always)]
    pub fn encode_into_vec(&self, src: &[u8]) -> Vec<u8> {
        encode(self.table, src)
    }

    ///Decoding function returns vector with data written.
    ///
    ///Requires feature `alloc`.
    ///
    ///# Arguments
    ///
    ///- `src` - Input to decode;
    ///
    ///# Result
    ///
    ///Returns `Some` if successful, containing decoded output
    ///
    ///Returns `None` if `src` is invalid input.
    #[inline]
    pub fn decode_into_vec(&self, src: &[u8]) -> Option<Vec<u8>> {
        let mut required_len = decode_len(src);
        let mut result = Vec::with_capacity(required_len);
        unsafe {
            let ptr = core::ptr::NonNull::new_unchecked(result.as_mut_ptr());
            match raw::decode_inner_with_rev(&self.reverse, src, ptr, &mut required_len) {
                true => {
                    result.set_len(required_len);
                },
                false => return None,
            }
        }

        Some(result)
    }
}
