#[test]
fn example() {
    let a = super::hex_to_vec("1c0111001f010100061a024b53535009181c");
    let b = super::hex_to_vec("686974207468652062756c6c277320657965");
    let c = super::hex_to_vec("746865206b696420646f6e277420706c6179");

    assert_eq!(super::fixed_xor(&a, &b), c);
}
