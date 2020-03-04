struct MyStruct<'a, T> {
    my_ref: &'a T,
}

fn f1() {
    let n: u8 = 77;
    let my_struct = MyStruct::<u8> { my_ref: &n };
    println!("{}", my_struct.my_ref);
}

pub fn main() {
    f1()
}
