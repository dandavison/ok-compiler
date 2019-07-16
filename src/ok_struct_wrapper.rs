use std::fmt;

struct StrWrapper<'a>(&'a str);

impl fmt::Display for StrWrapper<'static> {
    fn fmt(&self, writer: &mut fmt::Formatter) -> fmt::Result {
        writeln!(writer, "{}", self.0)
    }
}

pub fn main() {
    let f = StrWrapper {
        0: "this should be printed",
    };
    println!("f: {}", f);
}
