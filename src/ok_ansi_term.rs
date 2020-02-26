extern crate ansi_term;
use ansi_term;
use ansi_term::Colour::{Blue, Yellow};

pub fn main() {
    let mut ansi_term_style = match Blue {
        style::NO_COLOR => ansi_term::Style::default(),
        color => to_ansi_color(color).normal(),
    };

    println!("{}", Blue.on(Yellow).paint("Blue on yellow!"));
}
