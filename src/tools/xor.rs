extern crate cryptopals;

#[cfg(not(test))]
fn main() {
    use std::env;
    use std::io::{stdin, stdout, Read, Write};
    use cryptopals::conversions::hex_to_string;
    use cryptopals::combine::xor_each;

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Requires one argument which is the key with which to do repeating key xor \
                over stdin's bytes");
    }
    let key: Vec<u8> = args.get(1).unwrap().bytes().collect();
    let mut stdin: Vec<u8> = stdin().bytes().map(|b| b.unwrap()).collect();
    if *stdin.last().unwrap() == 10 { // Linefeed at end EOF
        stdin.pop();
    }
    let mut stdout = stdout();
    write!(&mut stdout, "{}", hex_to_string(&xor_each(&stdin, &key))).unwrap();
    write!(&mut stdout, "\n").unwrap();
}
