#[derive(Debug)] // so it can be formatted with {:?}
struct MyStruct {
    k1: usize,
    k2: String,
}


fn make_a_struct() -> MyStruct {
    MyStruct {
        k1: 0,
        k2: "my_string".to_string(),
    }
}


pub fn main() {
    let my_struct = make_a_struct();
    println!("{:?}", my_struct);
}
