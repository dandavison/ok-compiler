#[derive(Debug)] // so it can be formatted with {:?}
struct MyStruct {
    k1: usize,
    k2: String,
}

fn f1() -> MyStruct {
    MyStruct {
        k1: 0,
        k2: "my_string".to_string(),
    }
}

pub fn main() {
    let my_struct = f1();
    println!("{:?}", my_struct);
}
