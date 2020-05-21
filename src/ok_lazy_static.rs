use lazy_static::lazy_static;

/// https://stackoverflow.com/questions/48114390/why-does-a-lazy-static-value-claim-to-not-implement-a-trait-that-it-clearly-impl

#[derive(Debug)]
pub struct MyStruct {
    pub a: u8,
}

lazy_static! {
    static ref MY_STATIC: MyStruct = MyStruct { a: 1 };
}

pub fn main() {
    println!("{:?}", &*MY_STATIC);
}
