// Crypto pals: http://cryptopals.com/sets/1/challenges/3/

#[test]
fn test_single_byte_xor_cipher() {
    use conversions::string_to_hex;
    use crack::find_xor_key;
    use combine::xor_byte;

    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = string_to_hex(s);
    let key = find_xor_key(&bytes);
    println!("{}", String::from_utf8(xor_byte(&bytes, key)).unwrap());
    assert_eq!(key, 88); // As not to give the solution away, just check we found the right xor.
}
