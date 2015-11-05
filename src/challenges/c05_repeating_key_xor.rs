// Crypto pals: http://cryptopals.com/sets/1/challenges/5/

#[test]
fn test_repeating_key_xor() {
    use conversions::hex_to_string;
    use combine::xor_each;

    let x = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                    a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    assert_eq!(expected, hex_to_string(&xor_each(x.as_bytes(), key.as_bytes())))
}
