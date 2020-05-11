fn f(s: &str) {
    match s.to_lowercase().as_ref() {
        "none" => println!("Got none"),
        s => println!("Got something else: {}", s),
    };
}

pub fn main() {
    f("NONE");
    f("SOMETHING");
}
