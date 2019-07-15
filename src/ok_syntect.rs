use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// https://github.com/trishume/syntect#example-code

fn print(lang: &str, code: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(lang).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.light"]);
    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
    }
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
}
