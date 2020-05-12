use std::io::{self, BufRead};

#[allow(dead_code)]
fn f1() {
    let input = b"Hello \xF0\x90\x80World";
    let output = String::from_utf8_lossy(input);
    println!("{}", output);
}

fn f2() {
    for (i, line) in io::stdin().lock().lines().enumerate() {
        println!("{}: {}", i, line.unwrap());
    }
}

pub fn main() {
    f2();
}
