#[test]
fn test_detect_single_character_xor() {
    use conversions::string_to_hex;
    use combine::xor_byte;
    use text::CharFreq;

    let content = include_str!("data/4.txt");

    let en = CharFreq::for_english();
    let mut found = None;
    for line in content.lines() {
        let mut best = 1000.0;
        for b in 0x00..0xff {
            let bytes = string_to_hex(line);
            let s = String::from_utf8(xor_byte(&bytes, b)).unwrap_or("".to_string());
            if s.len() > 0 {
                let mut c = CharFreq::new();
                c.count_all(&s);
                let d = c.dist(&en);
                if d < 1.0 && d < best {
                    found = Some(s);
                    best = d;
                }
            }
        }
        if found.is_some() {
            break
        };
    }
    assert_eq!(found, Some("Now that the party is jumping\n".to_string()));
}
