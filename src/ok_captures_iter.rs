use regex::Regex;

fn get_captures<'a>(text: &'a str) -> Vec<&'a str> {
    let mut captures = Vec::new();
    for caps in Regex::new("(a)").unwrap().captures_iter(text) {
        captures.push(caps.get(1).map(|m| m.as_str()).unwrap());
    }
    captures
}

pub fn main() {
    let text = "aaaa";
    println!("{:?}", get_captures(text))
}
