#[test]
fn example() {
    use crate::strutils::*;
    use crate::xor::*;

    let msg = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    assert_eq!(rep_key_xor(&string_to_bytes(msg), &string_to_bytes(key)),
               hex_to_bytes("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"));

    //println!("{}", rep_key_xor(msg.clone(), key.clone()));
    //println!("{}", hex_to_string(rep_key_xor(hex_to_str(rep_key_xor(msg.clone(), key.clone())), key.clone())));
}
