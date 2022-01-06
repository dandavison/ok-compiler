use lazy_static::lazy_static;
use std::borrow::{Cow, ToOwned};

/// https://stackoverflow.com/questions/48114390/why-does-a-lazy-static-value-claim-to-not-implement-a-trait-that-it-clearly-impl

#[derive(Debug)]
enum MyEnum {
    A,
}

impl ToOwned for MyEnum {
    type Owned = MyEnum;
    fn to_owned(&self) -> Self::Owned {
        MyEnum::A
    }
}

lazy_static! {
    static ref CACHED_MY_ENUM: MyEnum = MyEnum::A;
}

fn _compute_my_enum() -> MyEnum {
    MyEnum::A
}

fn get_my_enum() -> Cow<'static, MyEnum> {
    #[cfg(not(test))]
    {
        Cow::Borrowed(&*CACHED_MY_ENUM)
    }
    #[cfg(test)]
    {
        Cow::Owned(_compute_my_enum())
    }
}

pub fn main() {
    println!("{:?}", get_my_enum());
}
