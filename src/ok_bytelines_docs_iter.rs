use bytelines::*;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    // construct our iterator from our file input
    let file = File::open("./res/numbers.txt").unwrap();
    let lines = BufReader::new(file).byte_lines();

    // walk our lines using `for` syntax
    for line in lines.into_iter() {
        // do something with the line, which is Vec<u8>
        line = 1;
    }
}
