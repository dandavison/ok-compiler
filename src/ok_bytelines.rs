use bytelines::ByteLinesReader;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    let file = File::open("./src/ok_bytelines.rs").unwrap();
    let mut lines = BufReader::new(file).byte_lines();

    while let Some(line) = lines.next() {
        println!("{:?}", line);
    }
}
