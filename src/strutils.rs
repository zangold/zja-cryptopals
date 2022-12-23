use itertools::Itertools;

/// Converts a string slice from hex encoding to base64 encoding.
pub fn hex_to_base64(hex: &str) -> String {
    let mut iter = hex.chars();

    // convert a single value in 0..64 to a base64 char
    let bits6_to_64 = |x| -> char {
        match x as u8 {
            n if n < 26 => (b'A' + n) as char,
            n if n < 52 => (b'a' + n - 26) as char,
            n if n < 62 => (b'0' + n - 52) as char,
            62 => '+',
            63 => '/',
            _ => panic!("Invalid argument to bits_to_64"),
        }
    };

    let bits12_to_64 = |x| (bits6_to_64((x & !63) >> 6), bits6_to_64(x & 63));

    let mut base64 = String::new();

    let h2v = |h| hex_to_value(h) as usize;

    loop {
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c)) => {
                let v = (h2v(a) << 8) + (h2v(b) << 4) + h2v(c);
                let (c1, c2) = bits12_to_64(v);
                base64.push(c1);
                base64.push(c2);
            }
            (Some(a), Some(b), None) => {
                let v = (h2v(a) << 8) + (h2v(b) << 4);
                let (c1, c2) = bits12_to_64(v);
                base64.push(c1);
                base64.push(c2);
            }
            (Some(a), None, None) => {
                let v = h2v(a) << 2;
                let c = bits6_to_64(v);
                base64.push(c);
            }
            (_, _, _) => break,
        }
    }

    base64
}

pub fn hex_to_value(c: char) -> u8 {
    if ('0'..='9').contains(&c) {
        c as u8 - b'0'
    } else if ('a'..='f').contains(&c) {
        c as u8 - b'a' + 10
    } else {
        panic!("invalid hex char: {}", c);
    }
}

/// b must be in 0..16
pub fn to_hex_char(b: u8) -> char {
    assert!(b < 16);

    if b < 10 {
        (b'0' + b) as char
    } else {
        (b'a' + b - 10) as char
    }
}

pub fn to_hex_chars(b: u8) -> (char, char) {
    (to_hex_char(b >> 4), to_hex_char(b & 0xf))
}

pub fn hex_to_bytes(msg: &str) -> Vec<u8> {
    assert!(msg.len() % 2 == 0);

    msg.chars()
        .batching(|iter| {
            if let (Some(c1), Some(c2)) = (iter.next(), iter.next()) {
                Some((hex_to_value(c1) << 4) + hex_to_value(c2))
            } else {
                None
            }
        })
        .collect()
}

pub fn bytes_to_string(msg: &[u8]) -> String {
    msg.iter().map(|x| *x as char).collect()
}

pub fn string_to_bytes(msg: &str) -> Vec<u8> {
    msg.chars().map(|x| x as u8).collect()
}
