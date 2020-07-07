fn f() {
    println!("hello!");
}

macro_rules! my_macro {
    () => {
        f()
    };
}

mod child_module {
    // Do I have to import all the bits and pieces the macro needs at every site the macro is used,
    // or am I doing something wrong?
    use super::f;

    pub fn g() {
        my_macro!();
    }
}

pub fn main() {
    child_module::g();
}
