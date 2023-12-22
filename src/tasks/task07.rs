#[test]
fn example() {
    use crate::block::*;
    use crate::strutils::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let key = string_to_bytes("YELLOW SUBMARINE");
    let bufreader = BufReader::new(File::open("test_assets/07_message.txt").unwrap());
    let mut msg = Vec::<u8>::new();

    bufreader
        .lines()
        .for_each(|line| msg.append(&mut base64_to_bytes(&line.unwrap())));

    let plaintext = aes_128_ecb_decrypt(&msg, &key, true);

    assert_eq!(
        bytes_to_string(&plaintext),
        std::fs::read_to_string("test_assets/07_plaintext.txt").unwrap()
    );

    assert_eq!(aes_128_ecb_encrypt(&plaintext, &key), msg);
}

#[test]
fn test_aes_128_ecb() {
    use crate::block::*;
    use crate::strutils::*;

    let message = string_to_bytes("yellow submarine");
    let key = string_to_bytes("YELLOW SUBMARINE");

    let ciphertext = aes_128_ecb_encrypt(&message, &key);
    let plaintext = aes_128_ecb_decrypt(&ciphertext, &key, true);

    println!("{:?} {:?}", message, plaintext);

    assert_eq!(plaintext, message);
}

#[test]
fn test_aes_128_cbc() {
    use crate::block::*;
    use crate::strutils::*;

    let message = string_to_bytes("yellow submarineYELLOW SUBMARINE");
    let key = string_to_bytes("YELLOW SUBMARINE");

    let ciphertext = aes_128_cbc_encrypt(&message, &key, &[0u8; 16]);
    let plaintext = aes_128_cbc_decrypt(&ciphertext, &key);

    println!("ciphertext: {:?}", ciphertext);
    println!("{:?} {:?}", message, plaintext);

    assert_eq!(plaintext, message);
}
