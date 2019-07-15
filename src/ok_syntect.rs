use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// https://github.com/trishume/syntect#example-code

fn print(lang: &str, code: &str) {
    // Load these once at the start of your program
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let syntax = syntax_set.find_syntax_by_extension(lang).unwrap();
    let mut h = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.light"]);
    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &syntax_set);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
    println!("\x1b[0m");
}

pub fn main() {
    print(
        "rs",
        "
// hello
pub struct Wow { hi: u64 }
fn blah() -> u64 {}
",
    );
    print(
        "py",
        "
# hello
def f():
    1
",
    );
    print!("\n\n");
}
