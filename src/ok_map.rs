fn f(s: &str) -> &str {
    s
}

fn main() {
    for s in vec!["x".to_string()].iter().map(|s| f(s)) {
        println!("{}", s);
    }

    for s in vec!["x".to_string()].iter().map(f) {
        println!("{}", s);
    }
}
