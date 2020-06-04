fn f1(x: &mut u8) {
    *x += 1;
}

fn f1_caller(x: &mut u8) {
    f1(x);
    f1(x);
}

fn f2(x: &mut Option<&mut u8>) {
    if let Some(x) = x {
        **x += 1;
    }
}

fn f2_caller(x: &mut Option<&mut u8>) {
    f2(&mut x); //
    f2(&mut x); // value used here after move
}

pub fn main() {
    let mut i: u8 = 0;
    dbg!(i);
    f1(&mut i);
    dbg!(i);
    f1_caller(&mut i);
    dbg!(i);
    f2(&mut Some(&mut i));
    dbg!(i);
    f2_caller(&mut Some(&mut i));
    dbg!(i);
}
