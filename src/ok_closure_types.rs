use std::borrow::Cow;

pub fn main() {
    let t = "t";
    let closure: Box<dyn Fn(&str) -> Cow<str>> = if true {
        Box::new(|_| Cow::from(t))
    } else {
        Box::new(|s| Cow::from(s))
    };
}
