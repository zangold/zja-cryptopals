// implement CBC mode using task 7's source code and decrypt the given message.
#[test]
fn example() {
    use crate::block::*;
    use crate::strutils::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let bufreader = BufReader::new(File::open("test_assets/10_message.txt").unwrap());

    let mut base64 = String::new();
    bufreader
        .lines()
        .for_each(|line| base64.push_str(&line.unwrap()));

    let ciphertext = base64_to_bytes(&base64);
    let key = string_to_bytes("YELLOW SUBMARINE");

    assert!(base64.len() % 16 == 0);

    let plaintext = aes_128_cbc_decrypt(&ciphertext, &key);

    assert_eq!(
        bytes_to_string(&plaintext),
        std::fs::read_to_string("test_assets/10_plaintext.txt").unwrap()
    );
}
