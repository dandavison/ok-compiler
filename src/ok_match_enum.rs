enum Wrapper {
    A(u8),
    B(u8),
    C(u8),
    D(u8),
    Empty,
}
use Wrapper::*;

fn extract(wrapper: Wrapper) -> Option<u8> {
    match wrapper {
        A(n) => Some(n),
        B(n) => Some(n),
        C(n) => Some(n),
        D(n) => Some(n),
        Empty => None,
    }
}

pub fn main() {
    println!("{:?}", extract(A(1)));
}
