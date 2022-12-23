#[test]
fn example() {
    use crate::strutils::*;
    use openssl::symm::{decrypt, Cipher};
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let cipher = Cipher::aes_128_ecb();
    let key = string_to_bytes("YELLOW SUBMARINE");

    let bufreader = BufReader::new(File::open("test_assets/07_message.txt").unwrap());
    let mut msg = Vec::<u8>::new();

    bufreader
        .lines()
        .for_each(|line| msg.append(&mut base64_to_bytes(&line.unwrap())));

    let plaintext = decrypt(cipher, &key, None, &msg).unwrap();

    assert_eq!(
        bytes_to_string(&plaintext),
        std::fs::read_to_string("test_assets/07_plaintext.txt").unwrap()
    );
}
