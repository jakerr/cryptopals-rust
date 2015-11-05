#[test]
fn test_break_reapeating_key_xor() {
    use conversions::base64_to_hex;
    use combine::xor_each;
    use crack::{guess_key_size, find_repeated_xor_key};
    use text::CharFreq;
    use std::f32;

    let en = CharFreq::for_english();
    let cipherlines: Vec<&str> = include_str!("data/6.txt").lines().collect();
    let ciphertext: String = cipherlines.join("");
    let cipher = base64_to_hex(ciphertext);
    let guesses = guess_key_size(&cipher);

    let mut best_message = (f32::MAX, "".to_string());
    for guess in guesses {
        let key = find_repeated_xor_key(&cipher, guess);
        let message = xor_each(&cipher, &key);

        // lower score is better.
        let msg_string = String::from_utf8(message).unwrap();
        let score = en.dist_from_string(&msg_string);
        if score < best_message.0 {
            best_message = (score, msg_string);
        }
    }
    println!("{}", best_message.1);
    assert!(best_message.1.starts_with("I'm back and I'm ringin' the bell"));
}
