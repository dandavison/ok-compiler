use std::fmt::Display;
use std::io::{self, BufRead};

/// We want a generic function that takes any type that implements the
/// iterator interface and yields printable items.
fn process_lines<I>(lines: I)
where
    I: Iterator,
    I::Item: Display,
{
    for line in lines {
        println!("ok {}", line);
    }
}

pub fn main() {
    process_lines(io::stdin().lock().lines().map(|l| l.unwrap()));
    process_lines("line one\nline two\n".split("\n").map(String::from));
}
