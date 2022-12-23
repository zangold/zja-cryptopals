#[test]
fn example() {
    use crate::strutils::*;
    use crate::xor::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let bufreader = BufReader::new(File::open("test_assets/06_ciphertext.txt").unwrap());
    let mut msg = Vec::<u8>::new();

    bufreader
        .lines()
        .for_each(|line| msg.append(&mut base64_to_bytes(&line.unwrap())));

    // (usize, float) meaning key_size and normalized hamming distance between the first two blocks
    // of length key_size.
    let mut hd_key_size = (2..80)
        .map(|key_size| {
            (
                key_size,
                hamming_distance(&msg[0..key_size], &msg[key_size..key_size * 2]) as f64
                    / (key_size as f64),
            )
        })
        .collect::<Vec<_>>();

    // Sort here instead of finding the element with the lowest hamming distance, in case we need
    // to use the key_size with the 2nd or 3rd (...) lowest hamming distance instead.
    hd_key_size.sort_by(|(_, hd1), (_, hd2)| hd1.partial_cmp(hd2).unwrap());

    let try_key_size = |key_size| {
        let mut transposed = Vec::<Vec<u8>>::new();
        transposed.resize(key_size, Vec::<u8>::with_capacity(msg.len() / key_size));

        msg.iter()
            .enumerate()
            .for_each(|(index, &u)| transposed[index % key_size].push(u));

        let mut full_key = Vec::<u8>::new();
        let mut total_score = 0;

        for block in transposed {
            let (key, _, score) = xor_decode(&block);
            full_key.push(key);
            total_score += score;
        }

        (full_key, total_score)
    };

    let mut best_key = Vec::<u8>::new();
    let mut best_score = 0;

    for &(key_size, _) in &hd_key_size[0..10] {
        let (key, score) = try_key_size(key_size);

        if score > best_score {
            best_key = key;
            best_score = score;
        }
    }

    let expected_plaintext = std::fs::read_to_string("test_assets/06_plaintext.txt").unwrap();

    let decoded_message = bytes_to_string(&rep_key_xor(&msg, &best_key));
    assert_eq!(decoded_message, expected_plaintext);
}
