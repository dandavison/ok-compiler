use std::fmt;

struct StringWrapper(String);

impl fmt::Display for StringWrapper {
    fn fmt(&self, writer: &mut fmt::Formatter) -> fmt::Result {
        writeln!(writer, "{}", self.0)
    }
}

pub fn main() {
    let f = StringWrapper {
        0: "this should be printed".to_string(),
    };
    println!("f: {}", f);
}
