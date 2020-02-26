fn f1() {
    let s = "10";

    println!("s is {}", s);

    let u: u8 = s.parse().expect("s is not parseable as u8");

    println!("u={} as hex={:02x}", u, u);
}

fn f2() {
    let s = "10";

    println!(
        "s={} as hex={:02x}",
        s,
        s.parse::<u8>().expect("s is not parseable as u8")
    );
}

pub fn main() {
    f2()
}
