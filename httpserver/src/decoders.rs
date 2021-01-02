use crate::buffer::Buffered;
use std::borrow::Borrow;

pub struct FormPair {
    pub name: String,
    pub value: String,
}

pub fn form_url_decode(s: &str) -> Vec<FormPair> {
    // https://url.spec.whatwg.org/#urlencoded-parsing
    let sequences = s.split('&');
    let mut output = Vec::<FormPair>::new();
    for bytes in sequences {
        if bytes.is_empty() {
            continue;
        }
        let name = if bytes.starts_with('=') {
            ""
        } else if !bytes.contains('=') {
            bytes
        } else {
            &bytes[0..bytes.find('=').unwrap()]
        }.replace("+", " ");
        let name = url_decode(name.as_str());

        let value = if bytes.ends_with('=') {
            ""
        } else if !bytes.contains('=') {
            ""
        } else {
            &bytes[bytes.find('=').unwrap() + 1..]
        }.replace("+", " ");
        let value = url_decode(value.as_str());
        output.push(FormPair { name, value });
    }
    output
}

pub fn url_decode(s: &str) -> String {
    // https://url.spec.whatwg.org/#percent-decode
    let s = s.to_string();
    let raw_bytes = s.into_bytes();
    let mut decoded = Vec::<u8>::new();

    let mut raw_iter = raw_bytes.iter().buffer(2);
    let mut skip = 0;
    while let Some(b) = raw_iter.next() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        macro_rules! match_valid_hex_byte (
            ($b:expr) => (
                matches!($b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
            )
        );
        match b {
            0x25 =>
                if raw_iter.peek(0)
                    .map_or(false, |b|
                        match_valid_hex_byte!(b))
                    && raw_iter.peek(1)
                    .map_or(false, |b|
                        match_valid_hex_byte!(b))
                {
                    let unit1 = raw_iter.peek(0).unwrap();
                    let unit2 = raw_iter.peek(1).unwrap();
                    let decoded_hex_byte = u8::from_str_radix(
                        String::from_utf8(vec![*unit1, *unit2]).unwrap().as_str(),
                        16).unwrap();
                    decoded.push(decoded_hex_byte);
                    skip += 2;
                },
            other => decoded.push(*other),
        }
    }

    String::from_utf8(decoded).unwrap()
}
