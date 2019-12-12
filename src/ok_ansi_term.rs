extern crate ansi_term;
use ansi_term::Colour::{Black, Blue, Cyan, Fixed, Green, Purple, Red, Yellow};
use ansi_term::Style;

pub fn main() {
    println!("{}", Blue.on(Yellow).paint("Blue on yellow!"));
}
