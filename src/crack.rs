pub fn find_xor_key(m: &[u8]) -> u8 {
    use combine::xor_byte;
    use std::f32;
    use text::CharFreq;

    let en = CharFreq::for_english();

    // xor, distance from en, string
    let mut best = (0x0, f32::MAX);

    for b in 0x00..0xff {
        let s = String::from_utf8(xor_byte(m, b)).unwrap_or("".to_string());
        if s.len() > 0 {
            let mut c = CharFreq::new();
            c.count_all(&s);
            let d = c.dist(&en);
            if d < best.1 {
                best = (b, d);
            }
        }
    }
    best.0
}

pub fn find_repeated_xor_key(m: &[u8], key_size: usize) -> Vec<u8> {
    // rotate the message into blocks separated by keysize.
    let mut blocks: Vec<Vec<u8>> = vec!();
    let chunks = m.chunks(key_size);
    for chunk in chunks {
        for (i, b) in chunk.iter().enumerate() {
            if blocks.len() <= i {
                blocks.push(vec!(*b));
            } else if let Some(block) = blocks.get_mut(i) {
                block.push(*b)
            }
        }
    }
    blocks.iter().map(|v| find_xor_key(v)).collect()
}

pub fn guess_key_size(m: &[u8]) -> Vec<usize> {
    use measure::hamming;
    // (normalized hamming_distances, key_sizes) If a new entry is threshold percent
    // diff better than current average the vec is replaced.  if its within threshold above or
    // below it's appended.
    let mut best = (vec!(8.0 as f32), vec!(m.len() as usize));
    let threshold = 0.10;

    for guess in 2..40 {
        let mut chunks = m.chunks(guess);
        let mut total_diff = 0.0;
        let mut pairs = 0;
        while let (Some(a), Some(b)) = (chunks.next(), chunks.next()) {
            pairs += 1;
            total_diff += hamming(a, b) as f32;
        }
        let diff = (total_diff / pairs as f32) / guess as f32;
        let bestavg = best.0.iter().fold(0.0, |a, &x| a + x) / best.0.len() as f32;
        let percdiff = ((bestavg - diff) / bestavg).abs();

        if percdiff < threshold {
            best.0.push(diff);
            best.1.push(guess);
        } else if diff < bestavg {
            best = (vec!(diff), vec!(guess));
        }
    }
    best.1
}

#[test]
fn test_guess_key_size() {
    use combine::xor_each;

    let quote = "Alice was beginning to get very tired of sitting by her sister on the bank, and
        of having nothing to do: once or twice she had peeped into the book her sister was reading,
        but it had no pictures or conversations in it, `and what is the use of a book,' thought
        Alice `without pictures or conversation?' So she was considering in her own mind (as well
        as she could, for the hot day made her feel very sleepy and stupid), whether the pleasure
        of making a daisy-chain would be worth the trouble of getting up and picking the daisies,
        when suddenly a White Rabbit with pink eyes ran close by her.  There was nothing so very
        remarkable in that; nor did Alice think it so very much out of the way to hear the Rabbit
        say to itself, `Oh dear! Oh dear! I shall be late!' (when she thought it over afterwards,
        it occurred to her that she ought to have wondered at this, but at the time it all seemed
        quite natural); but when the Rabbit actually took a watch out of its waistcoat-pocket, and
        looked at it, and then hurried on, Alice started to her feet, for it flashed across her
        mind that she had never before seen a rabbit with either a waistcoat-pocket, or a watch to
        take out of it, and burning with curiosity, she ran across the field after it, and
        fortunately was just in time to see it pop down a large rabbit-hole under the hedge.";
    let msg = quote.as_bytes();

    let key = [0x11, 0x23, 0x3f, 0xf9, 0x82, 0x12, 0x99, 0x22];
    let cipher = xor_each(&msg, &key);
    assert!(guess_key_size(&cipher).contains(&key.len()));

    let key = [0x1, 0x99];
    let cipher = xor_each(&msg, &key);
    assert!(guess_key_size(&cipher).contains(&key.len()));

    let key = [0x1, 0x99, 0xf2, 0x90, 0x01, 0x65, 0x69, 0x82, 0x11, 0x20, 0x30];
    let cipher = xor_each(&msg, &key);
    assert!(guess_key_size(&cipher).contains(&key.len()));
}
