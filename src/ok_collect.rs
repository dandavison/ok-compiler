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

fn g(strings: &Iterator<Item = &str>) {
    for s in strings.into_iter() {
        println!("{}", s);
    }
}

fn gg() {
    g(&["a"].iter().copied());
    let v1: Vec<String> = vec!["b".to_string()];
    g(&v1.iter().map(String::as_ref));
}

pub fn main() {
    ff();
    gg(); // Does not compile
}
