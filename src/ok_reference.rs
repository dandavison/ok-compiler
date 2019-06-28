fn f1() {
    let mut s = String::from("gravity");
    let r: &String = &s; // An immutable reference to mutable data can be made (mutable data can be borrowed as immutable)
    println!("{}", r);
    s.push_str(" always"); // But mutating the data implies borrowing as mutable, hence invalidating r.

    // println!("{}", r);
    //
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //  --> src/ok_reference.rs:5:5
    //   |
    // 3 |     let r: &String = &s;
    //   |                      -- immutable borrow occurs here
    // 4 |     println!("{}", r);
    // 5 |     s.push_str(" always");
    //   |     ^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
    // 6 |     println!("{}", r);
    //   |                    - immutable borrow later used here
}


fn f2() {
    let mut s = String::from("gravity"); // warning: variable does not need to be mutable
    let r: &String = &s;
    println!("{}", r);
    let mut ss = s.clone();
    ss.push_str(" always");
    println!("{}", r); // ok
}

pub fn main() {
    f1();
    f2();
}
