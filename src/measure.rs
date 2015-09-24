pub fn hamming(a: &[u8], b: &[u8]) -> u32 {
    a.iter().zip(b.iter()).fold(0, |acc, (a, b)| (a ^ b).count_ones() + acc)
}

#[test]
fn test_hamming() {
    let d = hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
    assert_eq!(d, 37);
}
