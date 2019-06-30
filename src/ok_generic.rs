use std::io::{self, BufRead};

/// We want a generic function that takes any type that implements the
/// iterator interface and yields printable items.
fn process_lines(lines: impl Iterator) -> io::Result<()> {
    for line in lines {
        println!("ok {}", line);
    }
    Ok(())
}


pub fn main() {
    process_lines(io::stdin().lock().lines()).unwrap();
}
