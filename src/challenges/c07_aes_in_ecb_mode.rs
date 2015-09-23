#[test]
fn test_can_decrypt() {
    use conversions::base64_to_hex;
    use openssl::crypto::symm::{Crypter, Type, Mode};

    const KEY: &'static str = "YELLOW SUBMARINE";
    let cipherlines: Vec<&str> = include_str!("data/7.txt").lines().collect();
    let ciphertext: String = cipherlines.connect("");
    let cipher = base64_to_hex(ciphertext);
    println!("Decoding {} char cipher", cipher.len());

    // Could just use openssl::crypto::symm::decrypt but lets practice using the richer Crypter
    // API.
    let c = Crypter::new(Type::AES_128_ECB);
    c.init(Mode::Decrypt, KEY.as_bytes(), &[]);
    c.pad(false);
    let mut msg = c.update(&cipher);
    msg.extend(c.finalize().iter());
    assert!(msg.len() > 0);
    let msg_string = String::from_utf8(msg).unwrap();
    println!("{}", msg_string);
    assert!(msg_string.starts_with("I'm back and I'm ringin' the bell"));
}
