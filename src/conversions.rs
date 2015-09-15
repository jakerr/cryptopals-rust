use std::iter::Iterator;

// Takes a hex digit between 0x0 and 0xf and returns the character '0'...'9' or
// 'a'...'f' that represents it.
fn hex_to_char(short: u8) -> char {
    match short {
        0x0...0x9 => (short + '0' as u8) as char,
        0xa...0xf => (short - 0xa + 'a' as u8) as char,
        _ => panic!("hex_to_char only converts short values between 0x0 and 0xf")
    }
}

// Takes a character '0'...'9' or 'a'...'f' and returns a u8 that the hex digit represents.
fn char_to_hex(c: char) -> u8 {
    match c {
        '0'...'9' => (c as u8 - '0' as u8),
        'a'...'f' => 10 + (c as u8 - 'a' as u8),
        _ => panic!("char_to_hex only converts char values between '0' and 'f'")
    }
}

// Takes a slice of bytes and converts to a hex string composed of '0'...'9','a'...'f' characters.
pub fn hex_to_string(hex: &[u8]) -> String {
    let byte_strings: Vec<String> = hex.iter().map(|x| {
        let h = (x & 0xF0) >> 4;
        let l = x & 0x0F;
        format!("{}{}", hex_to_char(h), hex_to_char(l))
    }).collect();
    byte_strings.connect("")
}

// Takes a string with only '0'...'9','a'...'f' characters in it and converts to the represented vector of bytes.
pub fn string_to_hex(string: &str) -> Vec<u8> {
    let mut v = Vec::new();
    let mut cs = string.chars();
    loop {
        let pair = (cs.next(), cs.next());
        match pair {
            (Some(h), Some(l)) => {
                let h = char_to_hex(h);
                let l = char_to_hex(l);
                let byte = (h << 4) | l;
                v.push(byte);
            },
            (Some(_), None) => panic!("Strings need pairs (even numbers) of characters to be considered valid hex."),
            _ => break
        }
    }
    v
}

// Wrapper arround hex bytes that allows returning n-bits at a time (stride).
// stride must be less than or equal to 8.
struct Bits<'a> {
    hex:&'a [u8],
    idx: usize,
    bidx: usize,
    stride: usize,
}

impl <'a>Bits<'a> {
    fn new(wrap: &'a [u8], stride: usize) -> Bits<'a> {
        assert!(stride <= 8);
        Bits {
            hex: wrap,
            idx: 0,
            bidx: 0,
            stride: stride
        }
    }

    fn bite(&mut self, bits: usize) -> (usize, u8) {
        let remain = 8 - self.bidx;
        let ignore = if remain > bits { remain - bits } else { 0 };
        let read = remain - ignore;

        let mut byte;
        if let Some(b) = self.hex.get(self.idx) {
            byte = (b << self.bidx) >> (self.bidx + ignore);
        } else {
            return (0, 0);
        }

        self.bidx += read;
        assert!(self.bidx <= 8);

        if self.bidx == 8 {
            self.bidx = 0;
            self.idx += 1;
        }
        (read, byte)
    }
}

impl <'a>Iterator for Bits<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let mut need = self.stride;
        let (high_read, high_byte) = self.bite(need);
        if high_read == 0 {
            return None;
        }
        need = need - high_read;

        let (_, low_byte) = if need > 0 {
            self.bite(need)
        } else {
            (0, 0)
        };

        return Some((high_byte << need) | low_byte);
    }
}

const BASE_64: [char; 64] = [
    'A','B','C','D','E','F','G','H',
    'I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X',
    'Y','Z','a','b','c','d','e','f',
    'g','h','i','j','k','l','m','n',
    'o','p','q','r','s','t','u','v',
    'w','x','y','z','0','1','2','3',
    '4','5','6','7','8','9','+','/'
];

// Takes a slice of bytes and encodes it into a base64 string.
pub fn hex_to_base64(hex: &[u8]) -> String {
    let mut s = String::new();
    // Take six bits at a time from the array of bytes.
    // Base64 is a set of 4x6 bits producing 4 characters.
    let mut b = Bits::new(hex, 6);
    loop {
        let set = (b.next(), b.next(), b.next(), b.next());
        match set {
            (None, _, _, _) => break,
            // If first 6 bits are present, at lest two of next 6 are as well
            (Some(h1), Some(h2), m, l) => {
                s.push(BASE_64[h1 as usize]);
                s.push(BASE_64[h2 as usize]);
                match m { Some(m) => s.push(BASE_64[m as usize]), _ => s.push('=') };
                match l { Some(l) => s.push(BASE_64[l as usize]), _ => s.push('=') };
            },
            _ => unreachable!()
        }
    }
    s
}

fn base64_inverse(c: char) -> Option<u8> {
    BASE_64.iter().position(|x| c == *x).map(|x| x as u8)
}

// Takes a base64 encoded string and returns the vector of bytes that it decodes to.
pub fn base64_to_hex(string: String) -> Vec<u8> {
    let mut v = Vec::new();
    let mut chars = string.chars();
    loop {
        let set = (chars.next(), chars.next(), chars.next(), chars.next());
        match set {
            (None, _, _, _) => break,
            (Some(a), Some(b), Some(c), Some(d)) => {
                let mut h = base64_inverse(a).unwrap() << 2;
                let mut m = base64_inverse(b).unwrap();
                h |= m >> 4;
                m = m << 4;
                v.push(h);

                let mut l = 0;
                match base64_inverse(c) {
                    Some(c) => {
                        m |= c >> 2;
                        l = c << 6;
                        v.push(m)
                    },
                    _ => ()
                }
                match base64_inverse(d) {
                    Some(d) => {
                        l |= d;
                        v.push(l)
                    },
                    _ => ()
                }
            },
            _ => panic!("Invalid base64. Inproperly padded.")
        }
    }
    v
}

#[test]
fn test_hex_and_string() {
    let h = &[0x0, 0x12, 0x34, 0xab, 0xcd, 0xef, 0xf];
    let s = "001234abcdef0f";
    assert_eq!(hex_to_string(h), s);
    assert_eq!(string_to_hex(s), h);

    let h = &[0x0];
    let s = "00";
    assert_eq!(hex_to_string(h), s);
    assert_eq!(string_to_hex(s), h);

    let h = &[];
    let s = "";
    assert_eq!(hex_to_string(h), s);
    assert_eq!(string_to_hex(s), h);
}

#[test]
fn test_base64() {
    // From wikipedia base64 examples
    let b = &[0x4d, 0x61, 0x6e];
    let s = "TWFu".to_string();
    assert_eq!(hex_to_base64(b), s);
    assert_eq!(base64_to_hex(s), b);

    let b = "leasure.".as_bytes();
    let s = "bGVhc3VyZS4=".to_string();
    assert_eq!(hex_to_base64(b), s);
    assert_eq!(base64_to_hex(s), b);

    let b = "any carnal pleas".as_bytes();
    let s = "YW55IGNhcm5hbCBwbGVhcw==".to_string();
    assert_eq!(hex_to_base64(b), s);
    assert_eq!(base64_to_hex(s), b);

    let b = "Man is distinguished, not only by his reason, but by this singular passion from \
        other animals, which is a lust of the mind, that by a perseverance of delight \
        in the continued and indefatigable generation of knowledge, exceeds the short \
        vehemence of any carnal pleasure.".as_bytes();

    let s = "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
    IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
    dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
    dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
    ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=".to_string();

    assert_eq!(hex_to_base64(&b), s);
    assert_eq!(String::from_utf8(base64_to_hex(s)).unwrap(), String::from_utf8(b.to_owned()).unwrap());

    // Crypto pals: http://cryptopals.com/sets/1/challenges/1/
    let b = string_to_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let s = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(hex_to_base64(&b), s);
    assert_eq!(base64_to_hex(s), b);
}

#[test]
fn test_bits() {
    let h = &[0x12];
    let mut b = Bits::new(h, 8);
    assert_eq!(b.next(), Some(0x12));
    assert_eq!(b.next(), None);

    let mut b = Bits::new(h, 4);
    assert_eq!(b.next(), Some(0x1));
    assert_eq!(b.next(), Some(0x2));
    assert_eq!(b.next(), None);

    let h = &[0b11111101, 0b11111011];
    let mut b = Bits::new(h, 6);
    assert_eq!(b.next(), Some(0b111111));
    assert_eq!(b.next(), Some(0b011111));
    assert_eq!(b.next(), Some(0b101100));
    assert_eq!(b.next(), None);
}
