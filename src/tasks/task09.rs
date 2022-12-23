#[test]
fn example() {
    use crate::block::*;
    use crate::strutils::*;

    let msg = string_to_bytes("YELLOW_SUBMARINE");
    let expected = string_to_bytes("YELLOW_SUBMARINE\x04\x04\x04\x04");

    assert_eq!(pkcs_7_pad(msg, 20), expected);
}
