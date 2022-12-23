/// Implements PKCS#7 padding, as described in challenge 9.
pub fn pkcs_7_pad(mut block: Vec<u8>, pad_to: usize) -> Vec<u8> {
    if block.len() >= pad_to {
        return block;
    }

    let pad: u8 = (pad_to - block.len()).try_into().unwrap();

    while block.len() < pad_to {
        block.push(pad);
    }

    block
}
