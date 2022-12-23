#[test]
fn example() {
    use crate::strutils::*;
    use crate::xor::fixed_xor;

    let a = hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let b = hex_to_bytes("686974207468652062756c6c277320657965");
    let c = hex_to_bytes("746865206b696420646f6e277420706c6179");

    assert_eq!(fixed_xor(&a, &b), c);
}
