pub fn xor_each(source: &[u8], with:&[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    let xiter = with.iter().cycle();
    let pairs = source.iter().zip(xiter);
    for (a, b) in pairs {
        v.push(a ^ b);
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;
    use conversions::string_to_hex;
    use conversions::hex_to_string;

    #[test]
    fn test_fixed_xor() {
        // Crypto pals: http://cryptopals.com/sets/1/challenges/2/
        let x = xor_each(
            &string_to_hex("1c0111001f010100061a024b53535009181c"),
            &string_to_hex("686974207468652062756c6c277320657965")
        );
        assert_eq!(hex_to_string(&x), "746865206b696420646f6e277420706c6179");
    }
}

