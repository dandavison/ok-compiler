use std::borrow::Borrow;
use std::io::{self, BufRead};

/// We want a generic function that takes any type that implements the
/// iterator interface and yields printable items.
fn process_lines(lines: impl Iterator<Item = impl Borrow<str>>) {
    for line in lines {
        println!("ok {}", line);
    }
}

pub fn main() {
    process_lines(io::stdin().lock().lines().map(|l| l.unwrap()));
    process_lines("line one\nline two\n".split("\n"));
}
