/// Takes two equal-length buffers and produces their XOR combination.
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

fn try_decode(msg: &[u8], key: u8) -> Vec<u8> {
    msg.iter().map(|x| x ^ key).collect()
}

fn score(msg: &[u8]) -> usize {
    msg.iter()
        .map(|c| match *c {
            l if (b'a'..=b'z').contains(&l) => 5,
            l if (b'A'..=b'Z').contains(&l) => 2,
            b' ' => 5,
            _ => 0,
        })
        .sum()
}

/// Attempts to decode msg as a single-byte-xor ciphertext, with a heuristic that favours english
/// text. Returns a tuple containing the key, the decoded text, and the score it was assigned.
pub fn xor_decode(msg: &[u8]) -> (u8, Vec<u8>, usize) {
    let mut best = Vec::<u8>::new();
    let mut best_key = 0;
    let mut best_score = 0;

    for key in 0..255 {
        let decoded = try_decode(msg, key);
        let score = score(&decoded);

        if score > best_score {
            best = decoded;
            best_key = key;
            best_score = score;
        }
    }

    (best_key, best, best_score)
}

/// Repeating key xor cipher
pub fn rep_key_xor(msg: &[u8], key: &[u8]) -> Vec<u8> {
    msg.iter()
        .enumerate()
        .map(|(index, ch)| ch ^ key[index % key.len()])
        .collect()
}

/// Find the hamming distance between v1 and v2 (which must be of equal length).
pub fn hamming_distance(v1: &[u8], v2: &[u8]) -> usize {
    assert_eq!(v1.len(), v2.len());

    v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| (a ^ b).count_ones() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strutils::*;

    #[test]
    fn test_hamming_distance() {
        let v1 = string_to_bytes("this is a test");
        let v2 = string_to_bytes("wokka wokka!!!");

        assert_eq!(hamming_distance(&v1, &v2), 37);
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(
            bytes_to_string(&base64_to_bytes("dGhpcyBpcyBhIHRlc3Qgc3RyaW5nCg==")),
            "this is a test string\n".to_string()
        );

        assert_eq!(
            bytes_to_string(&base64_to_bytes("eWV0IGFub3RoZXIgdGVzdCBzdHJpbmcK")),
            "yet another test string\n".to_string()
        );

        assert_eq!(
            bytes_to_string(&base64_to_bytes("eWV0IGFub3RoZXIgYmFzZTY0IHN0cmluZyEhIQo=")),
            "yet another base64 string!!!\n".to_string()
        );
    }
}
