use bytelines::ByteLinesReader;
use std::fs::File;
use std::io::{self, BufReader};

fn f1() {
    let file = File::open("./src/ok_bytelines.rs").unwrap();
    let mut lines = BufReader::new(file).byte_lines();

    while let Some(line) = lines.next() {
        println!("{:?}", String::from_utf8_lossy(line.unwrap()));
    }
}

fn f2() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().byte_lines();
    while let Some(line) = lines.next() {
        println!("{:?}", line.unwrap());
    }
}

pub fn main() {
    f1();
    f2();
}
