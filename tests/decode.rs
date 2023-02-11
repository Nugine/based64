use based64::{STANDARD_TABLE, URL_TABLE, PAD};
use based64::decode;

#[test]
fn handling_pad_middle() {
    ///Parsing stops the moment we meet padding character.
    ///So even though it repeats the same content twice, we only parse first, stopping, and
    ///returning result immediately.
    const INPUT: &str =  "SGVsbG8gV29ybGQ=SGVsbG8gV29ybGQ=";
    const EXPECTED: &str = "Hello World";

    let mut buffer = [0u8; 64];
    assert_eq!(decode(STANDARD_TABLE, INPUT.as_bytes(), &mut buffer), Some(EXPECTED.len()));
    assert_eq!(&buffer[..EXPECTED.len()], EXPECTED.as_bytes());
    assert_eq!(decode(URL_TABLE, INPUT.as_bytes(), &mut buffer), Some(EXPECTED.len()));
    assert_eq!(&buffer[..EXPECTED.len()], EXPECTED.as_bytes());
}

#[test]
fn should_fail_on_invalid_char() {
    const INPUT: [u8; 4] = [b'n', 0xff, b'A', PAD];

    let mut buffer = [0u8; 64];
    assert_eq!(decode(STANDARD_TABLE, &INPUT, &mut buffer), None);
    assert_eq!(decode(URL_TABLE, &INPUT, &mut buffer), None);
}

#[test]
fn should_correctly_decode_single_chunk_padded() {
    const INPUT: [(&str, &str); 3] = [
        ("QQ==", "A"),
        ("QUE=", "AA"),
        ("QUFB", "AAA"),
    ];

    let mut buffer = [0u8; 4];

    for (input, expected) in INPUT {
        let size = decode(STANDARD_TABLE, input.as_bytes(), &mut buffer).expect("to decode padded chunk");
        assert_eq!(&buffer[..size], expected.as_bytes());
        let size = decode(URL_TABLE, input.as_bytes(), &mut buffer).expect("to decode padded chunk");
        assert_eq!(&buffer[..size], expected.as_bytes());
    }
}

#[test]
fn should_handle_decode_padding_only() {
    let mut buffer = [0u8; 90];
    for idx in 1..buffer.len() {
        let input = "=".repeat(idx);
        assert_eq!(decode(STANDARD_TABLE, input.as_bytes(), &mut buffer), Some(0));
        assert_eq!(decode(URL_TABLE, input.as_bytes(), &mut buffer), Some(0));
    }
}

#[test]
fn should_decode() {
    const INPUT: [(&str, &str); 7] = [
        ("Lg==", "."),
        ("LA==", ","),
        ("VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wZWQgb3ZlciB0aGUgbGF6eSBkb2dzLg==", "The quick brown fox jumped over the lazy dogs."),
        ("SXQgd2FzIHRoZSBiZXN0IG9mIHRpbWVzLCBpdCB3YXMgdGhlIHdvcnN0IG9mIHRpbWVzLg==", "It was the best of times, it was the worst of times."),
        ("QWFCYkNjRGRFZUZmR2dIaElpSmpLa0xsTW1Obk9vUHBRcVJyU3NUdFV1VnZXd1h4WXlaeg==", "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz"),
        ("eHl6enkh", "xyzzy!"),
        ("aHR0cDovL2pha2FydGEuYXBhY2hlLm9yZy9jb21tbW9ucw==", "http://jakarta.apache.org/commmons"),
    ];

    let mut buffer = [0u8; 1024];
    for (input, expected) in INPUT {
        let size = decode(STANDARD_TABLE, input.as_bytes(), &mut buffer).expect("decode valid base64");
        let result = core::str::from_utf8(&buffer[..size]).unwrap();
        assert_eq!(result, expected);
    }

}

#[cfg(feature = "alloc")]
#[test]
fn should_decode_big_unpadded() {
    fn generate_b64_data(size: usize) -> String {
        fn match_idx(idx: usize) -> char {
            match (idx % 64) as u8 {
                v @ 0..=25 => (v + 'A' as u8) as char,
                v @ 26..=51 => (v - 26 + 'a' as u8) as char,
                v @ 52..=61 => (v - 52 + '0' as u8) as char,
                62 => '+',
                _ => '/',
            }
        }

        (0..size).map(match_idx).collect()
    }

    for idx in 100_000..=100_900 {
        let input = generate_b64_data(idx);
        based64::vec::decode(STANDARD_TABLE, input.as_bytes()).unwrap();
    }
}
