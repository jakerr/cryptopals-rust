use std::collections::HashMap;

#[derive(Debug)]
pub struct CharFreq {
    counts: HashMap<char, usize>,
    total: usize
}

impl CharFreq {
    pub fn for_english() -> CharFreq {
        CharFreq {
            counts: vec![
                (' ', 12802),
                ('e', 12702),
                ('t', 9056),
                ('a', 8167),
                ('o', 7507),
                ('i', 6966),
                ('n', 6749),
                ('s', 6327),
                ('h', 6094),
                ('r', 5987),
                ('d', 4253),
                ('l', 4025),
                ('c', 2782),
                ('u', 2758),
                ('m', 2406),
                ('w', 2361),
                ('f', 2228),
                ('g', 2015),
                ('y', 1974),
                ('p', 1929),
                ('b', 1492),
                ('v', 978),
                ('k', 772),
                ('j', 153),
                ('x', 150),
                ('q', 95),
                ('z', 74),
            ].into_iter().collect(),
            total: 100000
        }
    }

    pub fn new() -> CharFreq {
        CharFreq {
            counts: HashMap::new(),
            total: 0
        }
    }

    pub fn count(&mut self, c: char) {
        let c = c.to_lowercase().next().unwrap();
        let count = self.counts.entry(c).or_insert(0);
        *count += 1;
        self.total += 1;
    }

    pub fn count_all(&mut self, s: &str) {
        for c in s.chars() {
            self.count(c);
        }
    }

    pub fn dist(&self, other: &Self) -> f32 {
        let total = self.total as f32;
        let other_total = other.total as f32;

        let mut diff = 0.0;
        for (k,v) in self.counts.iter() {
            let p = *v as f32 / total;
            let op = *other.counts.get(k).unwrap_or(&0) as f32 / other_total;
            diff += (p - op).abs();
        }
        // Append to diff 'other' chars that never occur in this freq as well.
        for (k,v) in other.counts.iter() {
            if self.counts.contains_key(k) { continue }
            let p = *v as f32 / other_total;
            diff += p;
        }
        diff
    }

    pub fn dist_from_string(&self, s: &str) -> f32 {
        let mut other = CharFreq::new();
        other.count_all(s);
        self.dist(&other)
    }
}

#[test]
fn test_freq() {
    let mut a = CharFreq::new();
    a.count_all("aab");
    println!("{:?}", a);
    assert_eq!(a.total, 3);
    assert_eq!(a.counts.get(&'a'), Some(&2));
    assert_eq!(a.counts.get(&'b'), Some(&1));
}

#[test]
fn test_dist() {
    let mut a = CharFreq::new();
    let mut b = CharFreq::new();
    a.count_all("aab"); // a = 2/3, b = 1/3
    b.count_all("ab");  // a = 1/2, b = 1/2
    assert_eq!(a.dist(&b), b.dist(&a));
    assert_eq!(a.dist(&b), (2.0/3.0 - 1.0/2.0) + (1.0/2.0 - 1.0/3.0));

    let mut a = CharFreq::new();
    let mut b = CharFreq::new();
    a.count_all("abc");
    b.count_all("abc");
    assert!(a.dist(&b) == 0.0);
}

#[test]
fn test_english() {
    let en = CharFreq::for_english();
    assert_eq!(*en.counts.get(&'e').unwrap() as f32 / en.total as f32, 0.12702);

    let mut text = CharFreq::new();
    let mut gibberish = CharFreq::new();
    text.count_all("this is a test to see that real english is closer to the english distribution than some gibberish of the same length");
    gibberish.count_all("guvf vf n grfg gb frr gung erny ratyvfu vf pybfre gb gur ratyvfu qvfgevohgvba guna fbzr tvoorevfu bs gur fnzr yratgu");
    assert!(text.dist(&en) < (gibberish.dist(&en) - 0.5 /* much closer in other words */));
}


// Crypto pals: http://cryptopals.com/sets/1/challenges/3/
#[test]
fn test_single_byte_xor_cipher() {
    use conversions::string_to_hex;
    use crack::find_xor_key;

    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = string_to_hex(s);
    assert_eq!(find_xor_key(&bytes), 88); // As not to give the solution away, just check we found the right xor.
}
