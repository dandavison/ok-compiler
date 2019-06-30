use std::io::{self, BufRead};

fn process_lines() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        println!("ok {}", line?);
    }
    Ok(())
}


pub fn main() {
    process_lines().unwrap();
}
