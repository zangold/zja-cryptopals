#[test]
fn example() {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };

    use crate::strutils::*;
    use crate::xor::*;

    let buf_reader = BufReader::new(File::open("test_assets/04_strings.txt").unwrap());

    let mut best = String::new();
    let mut best_decoded = Vec::<u8>::new();
    let mut best_score = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let (_, decoded, score) = xor_decode(&hex_to_bytes(&line));

        if score > best_score {
            best = line;
            best_decoded = decoded;
            best_score = score;
        }
    }

    assert_eq!(
        best,
        "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f"
    );

    assert_eq!(
        bytes_to_string(&best_decoded),
        "Now that the party is jumping\n"
    )
}
