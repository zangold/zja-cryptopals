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

/// Converts a single base64 character to an integer (in 0..64).
pub fn base64_to_value(c: char) -> u8 {
    match c as u8 {
        l if (b'A'..=b'Z').contains(&l) => l - b'A',
        l if (b'a'..=b'z').contains(&l) => l - b'a' + 26,
        l if (b'0'..=b'9').contains(&l) => l - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => panic!("Invalid base64 value"),
    }
}

/// Converts a (padded) base64 string to its byte representation.
pub fn base64_to_bytes(base64: &str) -> Vec<u8> {
    assert!(base64.len() % 4 == 0);

    let mut vec = Vec::<u8>::new();

    let b2v = |x| base64_to_value(x) as usize;

    let mut to_chars = |a, b, c, d| {
        if c == '=' {
            // represents 8 bits
            let v = (b2v(a) << 2) + (b2v(b) >> 4);

            vec.push(v as u8);
        } else if d == '=' {
            // represents 16 bits
            let v = (b2v(a) << 10) + (b2v(b) << 4) + (b2v(c) >> 2);

            vec.push(((v & 0x00ff00) >> 8) as u8);
            vec.push((v & 0x0000ff) as u8);
        } else {
            // represents 24 bits
            let v = (b2v(a) << 18) + (b2v(b) << 12) + (b2v(c) << 6) + b2v(d);

            vec.push(((v & 0xff0000) >> 16) as u8);
            vec.push(((v & 0x00ff00) >> 8) as u8);
            vec.push((v & 0x0000ff) as u8);
        }
    };

    let mut it = base64.chars();

    while let (Some(a), Some(b), Some(c), Some(d)) = (it.next(), it.next(), it.next(), it.next()) {
        to_chars(a, b, c, d);
    }

    vec
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
