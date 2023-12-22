#![allow(dead_code)]
#![allow(unused_imports)]

use crate::block::*;
use crate::strutils::string_to_bytes;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum EncryptionType {
    Ecb,
    Cbc,
}

// Returns the encryption type used for verification purposes
fn encryption_oracle(plaintext: &[u8]) -> (EncryptionType, Vec<u8>) {
    // generate a random key
    let mut rng = rand::thread_rng();

    let key = (0..16).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

    let mut modified_plaintext = vec![];

    // Add 5-10 random bytes before the plaintext
    (0..rng.gen::<u8>() % 6 + 5).for_each(|_| modified_plaintext.push(rng.gen()));

    // Add in the plaintext
    modified_plaintext.extend_from_slice(plaintext);

    // Add 5-10 random bytes after the plaintext
    (0..rng.gen::<u8>() % 6 + 5).for_each(|_| modified_plaintext.push(rng.gen()));

    let padded_len = modified_plaintext.len() + 16 - (modified_plaintext.len() % 16);
    pkcs_7_pad(&mut modified_plaintext, padded_len);

    // Decide which type of encryption we're going to use
    let encryption = match rng.gen::<u8>() % 2 {
        0 => EncryptionType::Ecb,
        1 => EncryptionType::Cbc,
        _ => panic!("shouldn't get here"),
    };

    let iv = (0..16).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

    // encrypt the message
    let ciphertext = match encryption {
        EncryptionType::Ecb => aes_128_ecb_encrypt(&modified_plaintext, &key),
        EncryptionType::Cbc => aes_128_cbc_encrypt(&modified_plaintext, &key, &iv),
    };

    (encryption, ciphertext)
}

fn find_encryption_id(ciphertext: &[u8]) -> EncryptionType {
    // check the 2nd, 3rd, and 4th blocks against each other
    for i in 0..16 {
        let a = ciphertext[i + 16];
        let b = ciphertext[i + 32];
        let c = ciphertext[i + 48];

        if a != b || b != c {
            return EncryptionType::Cbc;
        }
    }

    EncryptionType::Ecb
}

#[test]
pub fn example() {
    for _ in 0..50 {
        // Repeat the plaintext many times, then look for repeated blocks in the output to see if we
        // can find ECB. The function above adds 5-10 bytes before this string, but the 2nd block
        // should be equal to the 3rd and 4th blocks if ECB encryption is being used.
        let plaintext = string_to_bytes(
            "YellowSubmarine YellowSubmarine YellowSubmarine YellowSubmarine YellowSubmarine ",
        );

        let (used_encryption, ciphertext) = encryption_oracle(&plaintext);
        let found_encryption = find_encryption_id(&ciphertext);

        assert_eq!(used_encryption, found_encryption);
    }
}
