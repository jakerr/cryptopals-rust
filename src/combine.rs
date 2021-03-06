pub fn xor_each(source: &[u8], with: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    let xiter = with.iter().cycle();
    let pairs = source.iter().zip(xiter);
    for (a, b) in pairs {
        v.push(a ^ b);
    }
    v
}

pub fn xor_byte(source: &[u8], with: u8) -> Vec<u8> {
    xor_each(source, &[with])
}

#[cfg(test)]
mod test {
    use super::*;
    use conversions::string_to_hex;

    #[test]
    fn test_single_byte_xor() {
        let x = xor_byte(&string_to_hex("4f7221752667267274677627"), 6);
        assert_eq!(String::from_utf8(x).unwrap(), "It's a trap!");
    }
}
