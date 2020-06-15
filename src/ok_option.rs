pub fn main() {
    let n = Some(0);
    n.map(|mut _n| _n += 1);
    println!("{:?}", n);
}
