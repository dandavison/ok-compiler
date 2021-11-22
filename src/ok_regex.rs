use regex::Regex;

pub fn main() {
    let re = Regex::new(r"\w").unwrap();
    println!("regex: {:?}\n", re);
    for c in &['a', '_', '|', '-', ':', '=', '/', '\\', ' ', '7'] {
        println!("{} {}", c, re.is_match(&c.to_string()));
    }
}
