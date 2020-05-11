fn f(s: &str) -> String {
    match s.to_lowercase().as_ref() {
        "none" => "Got none",
        s => s,
    }
    .to_string()
}

pub fn main() {
    println!("{}", f("NONE"));
    println!("{}", f("SOMETHING"));
}
