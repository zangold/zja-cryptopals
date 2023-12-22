#[test]
fn example() {
    use crate::block::*;
    use crate::strutils::*;

    let mut msg = string_to_bytes("YELLOW_SUBMARINE");
    let expected = string_to_bytes("YELLOW_SUBMARINE\x04\x04\x04\x04");

    pkcs_7_pad(&mut msg, 20);

    assert_eq!(msg, expected);
}
