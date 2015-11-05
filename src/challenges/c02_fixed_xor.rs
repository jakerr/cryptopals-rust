// Crypto pals: http://cryptopals.com/sets/1/challenges/2/

#[test]
fn test_fixed_xor() {
    use combine::xor_each;
    use conversions::{string_to_hex, hex_to_string};
    let x = xor_each(&string_to_hex("1c0111001f010100061a024b53535009181c"),
                     &string_to_hex("686974207468652062756c6c277320657965"));
    assert_eq!(hex_to_string(&x), "746865206b696420646f6e277420706c6179");
}
