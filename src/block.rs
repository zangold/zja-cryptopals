use crate::xor::*;

/// Implements PKCS#7 padding, as described in challenge 9.
pub fn pkcs_7_pad(block: &mut Vec<u8>, pad_to: usize) {
    if block.len() >= pad_to {
        return;
    }

    let pad: u8 = (pad_to - block.len()).try_into().unwrap();

    while block.len() < pad_to {
        block.push(pad);
    }
}

pub fn remove_padding(message: &mut Vec<u8>) {
    // first, determine if the end is actually padded
    let pad = *message.last().unwrap() as usize;

    if pad >= 17 || pad > message.len() {
        return;
    }

    for p in &message[message.len() - pad..] {
        if *p as usize != pad {
            return;
        }
    }

    message.truncate(message.len() - pad);
}

pub fn aes_128_cbc_decrypt(mut ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    const BLOCK_LEN: usize = 16;

    assert!(ciphertext.len() % BLOCK_LEN == 0);

    let mut iv = [0u8; BLOCK_LEN];
    let mut plaintext = Vec::<u8>::new();

    while !ciphertext.is_empty() {
        let mut plainblock = fixed_xor(
            &aes_128_ecb_decrypt(&ciphertext[0..BLOCK_LEN], key, false)[0..BLOCK_LEN],
            &iv,
        );

        plaintext.append(&mut plainblock);

        iv.clone_from_slice(&ciphertext[0..BLOCK_LEN]);
        ciphertext = &ciphertext[BLOCK_LEN..];
    }

    remove_padding(&mut plaintext);

    plaintext
}

pub fn aes_128_cbc_encrypt(mut plaintext: &[u8], key: &[u8], input_iv: &[u8]) -> Vec<u8> {
    const BLOCK_LEN: usize = 16;

    assert!(plaintext.len() % BLOCK_LEN == 0);

    let mut iv = [0u8; BLOCK_LEN];
    iv.clone_from_slice(input_iv);

    let mut ciphertext = Vec::<u8>::new();

    while !plaintext.is_empty() {
        let mut cipherblock = aes_128_ecb_encrypt(&fixed_xor(&iv, &plaintext[0..BLOCK_LEN]), key)
            [0..BLOCK_LEN]
            .to_vec();

        iv.clone_from_slice(&cipherblock[..]);
        ciphertext.append(&mut cipherblock);
        plaintext = &plaintext[BLOCK_LEN..];
    }

    ciphertext
}

pub fn aes_128_ecb_encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    use openssl::symm::{Cipher, Crypter, Mode};

    //encrypt(Cipher::aes_128_ecb(), &key, None, &ciphertext).unwrap()

    let mut padded_plaintext = plaintext.to_vec();
    pkcs_7_pad(
        &mut padded_plaintext,
        plaintext.len() + 16 - (plaintext.len() % 16),
    );

    let mut ciphertext = vec![0u8; padded_plaintext.len() + 16];

    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None).unwrap();
    encrypter.pad(false);

    let mut count = 0;

    while count + 16 <= padded_plaintext.len() {
        count += encrypter
            .update(
                &padded_plaintext[count..count + 16],
                &mut ciphertext[count..],
            )
            .unwrap();
    }

    count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count);

    assert!(ciphertext.len() % 16 == 0);

    ciphertext
}

pub fn aes_128_ecb_decrypt(ciphertext: &[u8], key: &[u8], padding: bool) -> Vec<u8> {
    //use openssl::symm::{decrypt, Cipher};

    //decrypt(Cipher::aes_128_ecb(), &key, None, &ciphertext).unwrap()
    use openssl::symm::{Cipher, Crypter, Mode};

    //encrypt(Cipher::aes_128_ecb(), &key, None, &ciphertext).unwrap()

    assert!(ciphertext.len() % 16 == 0);

    let mut plaintext = vec![0u8; ciphertext.len() + 16];

    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
    decrypter.pad(false);

    let mut count = 0;

    while count + 16 <= ciphertext.len() {
        count += decrypter
            .update(&ciphertext[count..count + 16], &mut plaintext[count..])
            .unwrap();
    }

    count += decrypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.truncate(count);

    if padding {
        remove_padding(&mut plaintext);
    }

    plaintext
}
