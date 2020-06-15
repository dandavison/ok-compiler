use bytelines::*;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    // construct our iterator from our file input
    let file = File::open("./res/numbers.txt").unwrap();
    let mut lines = BufReader::new(file).byte_lines();

    // walk our lines using `while` syntax
    while let Some(line) = lines.next() {
        // do something with the line, which is &[u8]
        line = 1;
    }
}
