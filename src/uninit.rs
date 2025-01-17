//! Functions to work with uninit memory

use core::{ptr, mem};

use super::Codec;

///Encoding function writing to uninit slice.
///
///# Arguments
///- `src` - Input to encode;
///- `dst` - Output to write;
///
///# Result
///
///Returns `Some` if successful, containing number of bytes written.
///
///Returns `None` if data cannot be encoded due to insufficient buffer size or size calculation overflow happens.
#[inline]
pub fn encode(table: &[u8; 64], src: &[u8], dst: &mut [mem::MaybeUninit<u8>]) -> Option<usize> {
    let mut len = dst.len();
    let dst = unsafe {
        ptr::NonNull::new_unchecked(dst.as_mut_ptr() as *mut u8)
    };
    match unsafe { super::raw::encode(table, src, dst, &mut len) } {
        true => Some(len),
        false => None,
    }
}

///Decoding function writing to uninit slice.
///
///# Arguments
///
///- `src` - Input to decode;
///- `dst` - Output to write;
///
///# Result
///
/// Returns `Some` if successful, containing number of bytes written.
///
/// Returns `None` if data cannot be encoded due to insufficient buffer size or invalid input.
#[inline]
pub fn decode(table: &[u8; 64], src: &[u8], dst: &mut [mem::MaybeUninit<u8>]) -> Option<usize> {
    let mut len = dst.len();
    let dst = unsafe {
        ptr::NonNull::new_unchecked(dst.as_mut_ptr() as *mut u8)
    };
    match unsafe { super::raw::decode(table, src, dst, &mut len) } {
        true => Some(len),
        false => None,
    }
}

impl<'a> Codec<'a> {
    ///Encoding function writing to uninit slice.
    ///
    ///# Arguments
    ///- `src` - Input to encode;
    ///- `dst` - Output to write;
    ///
    ///# Result
    ///
    ///Returns `Some` if successful, containing number of bytes written.
    ///
    ///Returns `None` if data cannot be encoded due to insufficient buffer size or size calculation overflow happens.
    #[inline(always)]
    pub fn encode_to_uninit(self, src: &[u8], dst: &mut [mem::MaybeUninit<u8>]) -> Option<usize> {
        encode(self.table, src, dst)
    }

    ///Decoding function writing to uninit slice.
    ///
    ///# Arguments
    ///
    ///- `src` - Input to decode;
    ///- `dst` - Output to write;
    ///
    ///# Result
    ///
    /// Returns `Some` if successful, containing number of bytes written.
    ///
    /// Returns `None` if data cannot be encoded due to insufficient buffer size or invalid input.
    #[inline]
    pub fn decode_to_uninit(&self, src: &[u8], dst: &mut [mem::MaybeUninit<u8>]) -> Option<usize> {
        let mut len = dst.len();
        let dst = unsafe {
            ptr::NonNull::new_unchecked(dst.as_mut_ptr() as *mut u8)
        };
        match unsafe { self.decode_to_raw(src, dst, &mut len) } {
            true => Some(len),
            false => None,
        }
    }
}
