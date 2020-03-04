struct MyStruct<'a, T> {
    my_u8: &'a u8,
    my_t: T,
}

fn f1() {
    let n: u8 = 77;
    let my_struct = MyStruct::<String> {
        my_t: "s".to_string(),
        my_u8: &n,
    };
    println!("{} {}", my_struct.my_t, my_struct.my_u8);
}

pub fn main() {
    f1()
}
