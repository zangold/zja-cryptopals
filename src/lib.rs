use itertools::Itertools;

// These modules contain tests corresponding to the tasks provided in the cryptopals challenges.
mod task01;
mod task02;

/// Converts a string slice from hex encoding to base64 encoding.
pub fn hex_to_base64(hex: &str) -> String {
    let mut iter = hex.chars();

    let hex_to_value = |x: char| -> usize {
        if ('0'..='9').contains(&x) {
            x as usize - '0' as usize
        } else if ('a'..='f').contains(&x) {
            x as usize - 'a' as usize + 10
        } else {
            0
        }
    };

    // convert a single value in 0..64 to a base64 char
    let bits6_to_64 = |x: usize| -> char {
        match x as u8 {
            n if n < 26 => (b'A' + n) as char,
            n if n < 52 => (b'a' + n - 26) as char,
            n if n < 62 => (b'0' + n - 52) as char,
            62 => '+',
            63 => '/',
            _ => panic!("Invalid argument to bits_to_64"),
        }
    };

    let bits12_to_64 =
        |x: usize| -> (char, char) { (bits6_to_64((x & !63) >> 6), bits6_to_64(x & 63)) };

    let mut base64 = String::new();

    loop {
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c)) => {
                let v = (hex_to_value(a) << 8) + (hex_to_value(b) << 4) + hex_to_value(c);
                let (c1, c2) = bits12_to_64(v);
                base64.push(c1);
                base64.push(c2);
            }
            (Some(a), Some(b), None) => {
                let v = (hex_to_value(a) << 8) + (hex_to_value(b) << 4);
                let (c1, c2) = bits12_to_64(v);
                base64.push(c1);
                base64.push(c2);
            }
            (Some(a), None, None) => {
                let v = hex_to_value(a) << 2;
                let c = bits6_to_64(v);
                base64.push(c);
            }
            (_, _, _) => break,
        }
    }

    base64
}

pub fn hex_to_value(c: Option<char>) -> usize {
    if let Some(x) = c {
        if ('0'..='9').contains(&x) {
            x as usize - '0' as usize
        } else if ('a'..='f').contains(&x) {
            x as usize - 'a' as usize + 10
        } else {
            0
        }
    } else {
        0
    }
}

pub fn hex_to_vec(msg: &str) -> Vec<u8> {
    msg.chars()
        .batching(|iter| {
            let (c1, c2) = (iter.next(), iter.next());

            if c1.is_none() && c2.is_none() {
                None
            } else {
                Some(((hex_to_value(c1) << 4) + hex_to_value(c2)) as u8)
            }
        })
        .collect()
}

/// Takes two equal-length buffers and produces their XOR combination.
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}
