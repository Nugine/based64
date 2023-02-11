//!BASE64 library for chads
//!
//!## Features
//!
//!- `alloc` - Enables usage of heap based collections;
//!
//!## API
//!
//!- [raw](raw) - Contains functions to work with raw pointers. Mostly unsafe.
//!- [uninit](uninit) - Contains functions to work with unintialized slices.
//!- [vec](vec) - Contains high level functions that returns `Vec`. Requires `alloc` feature.
//!- [string](string) - Contains high level functions that returns `String`. Requires `alloc` feature.
//!- [Codec](Codec) - Wrapper that allows to pre-built lookup table for decoding. Useful if you want to safe tiny bit on building lookup table.

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod raw;
pub mod uninit;
#[cfg(feature = "alloc")]
pub mod vec;
#[cfg(feature = "alloc")]
pub mod string;

use core::mem;

///Base64 padding character
pub const PAD: u8 = b'=';
///Default character table used by based64
pub static STANDARD_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
///Alternative table URL safe.
pub static URL_TABLE: &[u8; 64] = b"ABCDEFGHIJLKMNOPQRSTUVWXYZabcdefghijlkmnopqrstuvwxyz0123456789-_";
///Codec which uses `STANDARD_TABLE`
pub static STANDARD_CODEC: Codec<'static> = Codec::new(STANDARD_TABLE);
///Codec which uses `URL_TABLE`
pub static URL_CODEC: Codec<'static> = Codec::new(URL_TABLE);

#[inline]
///Validates custom character table by requiring user to provide table of specific size
///containing only ASCII characters.
pub const fn assert_valid_character_table(table: &[u8; 64]) -> bool {
    let mut idx = 0;
    while idx < table.len() {
        if !table[idx].is_ascii() {
            return false
        }

        idx += 1;
    }

    true
}

const REVERSE_TABLE_SIZE: usize = (u8::max_value() as usize) + 1;
#[inline(always)]
const fn build_reverse_table(table: &[u8; 64]) -> [i8; REVERSE_TABLE_SIZE] {
    let mut reverse_table = [-1i8; (u8::max_value() as usize) + 1];

    let mut idx = 0;
    loop {
        let byte = table[idx] as usize;
        reverse_table[byte] = idx as i8;
        idx += 1;

        if idx >= table.len() {
            break;
        }
    }

    reverse_table
}



#[inline(always)]
///Returns number of bytes necessary to encode input of provided size (including padding).
///
///On overflow returns wrapped value.
pub const fn encode_len(input: usize) -> usize {
    input.wrapping_mul(4).wrapping_div(3).wrapping_add(3) & !3
}

///Returns number of bytes necessary to decode provided input.
///
///In case of not rounded input, assumes the worst.
pub const fn decode_len(input: &[u8]) -> usize {
    let len = input.len();
    if len == 0 {
        return 0
    }

    let unused_num = len & 3;
    if unused_num != 0 {
        //unpadded (probably)
        len.wrapping_div(4).wrapping_mul(3) + unused_num
    } else {
        //padded so it is simply
        //len / 4 * 3
        let result = len.wrapping_div(4).wrapping_mul(3);
        if input[len - 1] != PAD {
            result
        } else if input[len - 2] != PAD {
            result - 1
        } else if input[len - 3] != PAD {
            result - 2
        } else {
            result - 3
        }
    }
}

///Encoding function writing to slice.
///
///# Arguments
///
///- `src` - Input to encode;
///- `dst` - Output to write;
///
///# Result
///
///Returns `Some` if successful, containing number of bytes written.
///
///Returns `None` if data cannot be encoded due to insufficient buffer size or size calculation overflow happens.
#[inline]
pub fn encode(table: &[u8; 64], src: &[u8], dst: &mut [u8]) -> Option<usize> {
    unsafe {
        uninit::encode(table, src, mem::transmute(dst))
    }
}

///Decoding function writing to slice.
///
///# Arguments
///
///- `src` - Input to decode;
///- `dst` - Output to write;
///
///# Result
///Returns `Some` if successful, containing number of bytes written.
///
///Returns `None` if data cannot be encoded due to insufficient buffer size or invalid input.
#[inline]
pub fn decode(table: &[u8; 64], src: &[u8], dst: &mut [u8]) -> Option<usize> {
    unsafe {
        uninit::decode(table, src, mem::transmute(dst))
    }
}


///BASE64 codec
#[derive(Copy, Clone)]
pub struct Codec<'a> {
    table: &'a [u8; 64],
    reverse: [i8; REVERSE_TABLE_SIZE]
}

impl<'a> Codec<'a> {
    #[inline(always)]
    ///Creates new codec, validating that table contains only ASCII characters.
    pub const fn new(table: &'a [u8; 64]) -> Self {
        assert!(assert_valid_character_table(table));
        Self {
            table,
            reverse: build_reverse_table(table),
        }
    }

    #[inline(always)]
    ///Access prebuilt instance of codec with `STANDARD_TABLE`
    pub fn standard() -> &'static Codec<'static> {
        &STANDARD_CODEC
    }

    #[inline(always)]
    ///Access prebuilt instance of codec with `URL_TABLE`
    pub fn url_usafe() -> &'static Codec<'static> {
        &URL_CODEC
    }
}

impl<'a> Codec<'a> {
    ///Encoding function writing to slice.
    ///
    ///# Arguments
    ///
    ///- `src` - Input to encode;
    ///- `dst` - Output to write;
    ///
    ///# Result
    ///
    ///Returns `Some` if successful, containing number of bytes written.
    ///
    ///Returns `None` if data cannot be encoded due to insufficient buffer size or size calculation overflow happens.
    #[inline(always)]
    pub fn encode_to(&self, src: &[u8], dst: &mut [u8]) -> Option<usize> {
        encode(self.table, src, dst)
    }

    ///Decoding function writing to slice.
    ///
    ///# Arguments
    ///
    ///- `src` - Input to decode;
    ///- `dst` - Output to write;
    ///
    ///# Result
    ///Returns `Some` if successful, containing number of bytes written.
    ///
    ///Returns `None` if data cannot be encoded due to insufficient buffer size or invalid input.
    #[inline]
    pub fn decode_to(&self, src: &[u8], dst: &mut [u8]) -> Option<usize> {
        unsafe {
            self.decode_to_uninit(src, mem::transmute(dst))
        }
    }
}
