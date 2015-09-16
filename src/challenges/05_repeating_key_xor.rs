extern crate cryptopals;

use std::env;
use std::io::{stdin, stdout, Read, Write};
use cryptopals::conversions::hex_to_string;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Requires one argument which is the key with which to do repeating key xor \
                over stdin's bytes");
    }
    let key = args.get(1).unwrap().bytes().cycle();
    let stdin = stdin().bytes().map(|b| b.unwrap());
    let mut stdout = stdout();
    for (a, b) in stdin.zip(key) {
        if a == 10 { continue } // Line feed
        write!(&mut stdout, "{}", hex_to_string(&[a ^ b]));
    }
    write!(&mut stdout, "\n");
}
