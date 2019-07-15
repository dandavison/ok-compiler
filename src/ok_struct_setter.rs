#[derive(Debug)]
struct MyStruct {
    k1: Option<usize>,
}

pub fn main() {
    let mut t = MyStruct { k1: Some(0) };
    println!("{}", t.k1.unwrap());
    t.k1 = Some(1);
    println!("{}", t.k1.unwrap());
    println!("{:?}", t);
}
