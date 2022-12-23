#![cfg(test)]

use crate::strutils::*;
use crate::xor::*;

#[test]
fn example() {
    let ciphertext =
        hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let plaintext = bytes_to_str(&xor_decode(&ciphertext));

    assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
}
