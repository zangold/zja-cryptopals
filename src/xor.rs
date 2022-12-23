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

pub fn xor_decode(msg: &[u8]) -> (Vec<u8>, usize) {
    let mut best = Vec::<u8>::new();
    let mut best_score = 0;

    for key in 0..255 {
        let decoded = try_decode(msg, key);
        let score = score(&decoded);

        if score > best_score {
            best = decoded;
            best_score = score;
        }
    }

    (best, best_score)
}

/// Repeating key xor cipher
pub fn rep_key_xor(msg: &[u8], key: &[u8]) -> Vec<u8> {
    msg.iter()
        .enumerate()
        .map(|(index, ch)| ch ^ key[index % key.len()])
        .collect()
}
