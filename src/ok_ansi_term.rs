use std::fmt::Write;

use ansi_term;
use ansi_term::Colour::{Red, Yellow};

fn f1() {
    println!("This is in red: {}", Red.paint("a red string"));
}

fn f2() {
    let style = ansi_term::Style::new().fg(Red).on(Yellow);
    println!(
        "This is red on yellow: {}",
        style.paint("A red on yellow string")
    );
}

fn f3() {
    let style = ansi_term::Style::new().fg(Red).on(Yellow);
    let mut ansi_strings = Vec::new();
    ansi_strings.push(style.paint("A red on yellow string rendered from ANSIStrings."));
    println!(
        "{} {}",
        ansi_term::ANSIStrings(&ansi_strings),
        "This shouldn\'t be colored A."
    );
    println!(
        "{} {}",
        ansi_term::ANSIStrings(&ansi_strings),
        "This shouldn\'t be colored B."
    );
}

fn f4() {
    let style = ansi_term::Style::new().fg(Red).on(Yellow);
    let mut ansi_strings = Vec::new();
    ansi_strings
        .push(style.paint("A red on yellow string rendered from ANSIStrings put in a buffer."));

    let mut output_buffer = String::new();

    write!(
        &mut output_buffer,
        "{} {}",
        ansi_term::ANSIStrings(&ansi_strings),
        "This shouldn\'t be colored."
    )
    .unwrap();
    println!("{}", &output_buffer);
}

fn f5() {
    let style = ansi_term::Style::new().on(Yellow);
    let mut ansi_strings = Vec::new();
    ansi_strings.push(style.paint("|"));
    ansi_strings.push(style.paint("|"));
    println!("{}", ansi_term::ANSIStrings(&ansi_strings));
}

pub fn main() {
    f5();
}
