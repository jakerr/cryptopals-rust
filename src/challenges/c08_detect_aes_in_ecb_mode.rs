#[test]
fn test_can_find_aes_ecb_cipher() {
    use conversions::string_to_hex;
    use std::collections::HashMap;

    let ciphers: Vec<Vec<u8>> = include_str!("data/8.txt")
                                    .lines()
                                    .map(|x| string_to_hex(x))
                                    .collect();

    let mut dupeidx = vec!();
    for cipher in ciphers.iter() {
        let mut block_counts: HashMap<Vec<u8>, usize> = HashMap::new();
        for block in cipher.chunks(16) {
            let count = block_counts.entry(block.to_owned()).or_insert(0);
            *count += 1;
        }
        let dupes = block_counts.iter().fold(0, |a, (_, count)| a + count - 1);
        dupeidx.push(dupes);
    }
    let mut found_idx = (0, 0); // idx, dupes
    for (idx, dupes) in dupeidx.iter().enumerate() {
        if *dupes > found_idx.1 {
            found_idx = (idx, *dupes)
        }
    }
    assert!(found_idx.1 > 0);
    assert_eq!(found_idx.0, 132);
    println!("Index of aes in ecb mode cipher is {}", found_idx.0);

    // Thought maybe an easter egg was the text was encrypted with same key as last time.
    // Does not seem to be the case. Maybe we can come back and try to crack this thing later.
    /*
    use openssl::crypto::symm::{decrypt, Type};
    const KEY: &'static str = "YELLOW SUBMARINE";
    let message = String::from_utf8(decrypt(Type::AES_128_ECB, KEY.as_bytes(),
                                            &[],
                                            ciphers.get(found_idx.0).unwrap())).unwrap();
    println!("Message is {}?", message);
    */
}
