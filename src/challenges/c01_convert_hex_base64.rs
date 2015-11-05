// Crypto pals: http://cryptopals.com/sets/1/challenges/1/

#[test]
fn test_convert_hex_to_base64() {
    use conversions::{base64_to_hex, hex_to_base64, string_to_hex};
    let b = string_to_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f\
                           6e6f7573206d757368726f6f6d");
    let s = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(hex_to_base64(&b), s);
    assert_eq!(base64_to_hex(s), b);
}
