//! Low level functions

use core::ptr::NonNull;
use super::{PAD, encode_len, decode_len};

#[cold]
#[inline(never)]
fn unlikely_false() -> bool {
    false
}

pub(crate) fn encode_inner(table: &[u8; 64], src: &[u8], dst: NonNull<u8>, len: &mut usize) {
    let mut it = src.as_ptr();
    let it_end = unsafe {
        it.add(src.len())
    };
    let mut cursor = dst.as_ptr();
    while (it_end as usize) - (it as usize) >= 3 {
        unsafe {
            *cursor = *table.get_unchecked(
                (*it).wrapping_shr(2) as usize
            );
            cursor = cursor.add(1);

            *cursor = *table.get_unchecked(
                (((*it) & 0x03).wrapping_shl(4) | (*it.add(1)).wrapping_shr(4)) as usize
            );
            cursor = cursor.add(1);

            *cursor = *table.get_unchecked(
                (((*it.add(1)) & 0x0f).wrapping_shl(2) | (*it.add(2)).wrapping_shr(6)) as usize
            );
            cursor = cursor.add(1);

            *cursor = *table.get_unchecked(
                ((*it.add(2)) & 0x3f) as usize
            );
            cursor = cursor.add(1);

            it = it.add(3);
        }
    }

    let remain_len = (it_end as usize) - (it as usize);
    if remain_len > 0 {
        unsafe {
            *cursor = *table.get_unchecked(
                (*it).wrapping_shr(2) as usize
            );
            cursor = cursor.add(1);

            if remain_len == 1 {
                *cursor = *table.get_unchecked(
                    ((*it) & 0x03).wrapping_shl(4) as usize
                );
                cursor = cursor.add(1);

                *cursor = PAD;
                cursor = cursor.add(1);
            } else {
                *cursor = *table.get_unchecked(
                    (((*it) & 0x03).wrapping_shl(4) | (*it.add(1)).wrapping_shr(4)) as usize
                );
                cursor = cursor.add(1);

                *cursor = *table.get_unchecked(
                    ((*it.add(1)) & 0x0f).wrapping_shl(2) as usize
                );
                cursor = cursor.add(1);
            }

            *cursor = PAD;
            cursor = cursor.add(1);
        }
    }

    *len = cursor as usize - dst.as_ptr() as usize;
}

#[inline]
///Raw encoding function.
///
///# Arguments
///
///- `src` - Input to encode;
///- `dst` - Output to write;
///- `len` - Output length, modified with required size regardless of outcome, unless calculation wrapping happens.
///
///# Result
///Returns `true` on success.
///
///Returns `false` if buffer overflow would to happen or required_len is too big.
pub unsafe fn encode(table: &[u8; 64], src: &[u8], dst: NonNull<u8>, len: &mut usize) -> bool {
    let required_len = encode_len(src.len());
    if required_len < src.len() {
        //bro, how likely is overflow?
        return unlikely_false();
    } else if required_len > *len {
        *len = required_len;
        return false;
    }

    encode_inner(table, src, dst, len);
    true
}

//reverse table is 0..=255
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

pub(crate) fn decode_inner(table: &[u8; 64], mut src: &[u8], dst: NonNull<u8>, len: &mut usize) -> bool {
    let mut cursor = dst.as_ptr();
    let mut chunk = [0u8; 4];
    let mut chunk_len = 0;

    let reverse_table = build_reverse_table(table);

    macro_rules! get_base64_byte {
        ($src:ident[$idx:literal]) => {{
            let ch = match $src.get($idx) {
                Some(ch) if *ch != PAD => *ch,
                _ => {
                    chunk_len = $idx - 1;
                    break;
                },
            };

            let pos = unsafe {
                reverse_table.get_unchecked(ch as usize)
            };
            if *pos == -1 {
                return unlikely_false()
            }
            *pos as u8
        }}
    }

    loop {
        chunk[0] = get_base64_byte!(src[0]);
        chunk[1] = get_base64_byte!(src[1]);
        chunk[2] = get_base64_byte!(src[2]);
        chunk[3] = get_base64_byte!(src[3]);

        unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
            *cursor = (chunk[1] & 0xf).wrapping_shl(4).wrapping_add((chunk[2] & 0x3c).wrapping_shr(2));
            cursor = cursor.add(1);
            *cursor = (chunk[2] & 0x3).wrapping_shl(6).wrapping_add(chunk[3]);
            cursor = cursor.add(1);
        }

        src = &src[4..];

        if src.is_empty() {
            break;
        }
    }

    match chunk_len {
        3 => unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
            *cursor = (chunk[1] & 0xf).wrapping_shl(4).wrapping_add((chunk[2] & 0x3c).wrapping_shr(2));
            cursor = cursor.add(1);
            *cursor = (chunk[2] & 0x3).wrapping_shl(6).wrapping_add(chunk[3]);
            cursor = cursor.add(1);
        },
        2 => unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
            *cursor = (chunk[1] & 0xf).wrapping_shl(4).wrapping_add((chunk[2] & 0x3c).wrapping_shr(2));
            cursor = cursor.add(1);
        },
        1 => unsafe {
            *cursor = chunk[0].wrapping_shl(2).wrapping_add((chunk[1] & 0x30).wrapping_shr(4));
            cursor = cursor.add(1);
        },
        _ => (),
    }

    *len = cursor as usize - dst.as_ptr() as usize;
    true
}

#[inline]
///Raw decoding function.
///
///# Arguments
///- `src` - Input to decode;
///- `dst` - Output to write;
///- `len` - Output length, modified with required size regardless of outcome.
///
///# Result
///Returns `true` on success.
///
///Returns `false` if buffer overflow would to happen or `src` is empty or invalid base64.
pub unsafe fn decode(table: &[u8; 64], src: &[u8], dst: NonNull<u8>, len: &mut usize) -> bool {
    let required_len = decode_len(src);

    if required_len == 0 {
        *len = 0;
        return true;
    }

    decode_inner(table, src, dst, len)
}
