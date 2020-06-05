fn f(strings: &[&str]) {
    for s in strings {
        println!("{}", s);
    }
}

fn ff() {
    let v1: Vec<String> = vec!["b".to_string()];
    let v2: Vec<&str> = v1.iter().map(String::as_ref).collect::<Vec<&str>>();
    let slice: &[&str] = v2.as_ref();
    f(slice);
}

pub fn main() {
    ff()
}
