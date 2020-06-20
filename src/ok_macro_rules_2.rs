fn f() {
    println!("hello!");
}

macro_rules! my_macro {
    () => {
        f()
    };
}

pub fn main() {
    my_macro!();
}
