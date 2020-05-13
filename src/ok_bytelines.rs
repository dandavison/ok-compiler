use bytelines::ByteLinesReader;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn f1() {
    let file = File::open("./src/ok_bytelines.rs").unwrap();
    let mut lines = BufReader::new(file).byte_lines();

    while let Some(line) = lines.next() {
        println!("{:?}", String::from_utf8_lossy(line.unwrap()));
    }
}

#[allow(dead_code)]
fn f2() {
    for (i, line) in io::stdin().lock().lines().enumerate() {
        println!("{}: {}", i, line.unwrap());
    }
}

pub fn main() {
    f1();
}
