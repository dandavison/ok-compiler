type BoxedClosure = Box<dyn Fn(u8) -> u8>;

fn get_closures() -> Vec<BoxedClosure> {
    vec![Box::new(|x: u8| x + 1), Box::new(|x: u8| x + 2)]
}

macro_rules! get_closures_macro_1 {
    ([ $( $i:expr ),* ]) => {
         vec![$( Box::new(|x: u8| x + $i) as BoxedClosure ),*]
    }
}

macro_rules! get_closures_macro_2 {
    ([ $( $ident:ident => $expr:expr ),* ]) => {
        vec![$( Box::new(|$ident: u8| $expr) as BoxedClosure ),*]
    }
}

pub fn main() {
    for f in get_closures() {
        println!("f(1) = {}", f(1));
    }
    println!();
    for f in get_closures_macro_1!([1, 2]) {
        println!("f(1) = {}", f(1));
    }
    println!();
    for f in get_closures_macro_2!([x => x + 1, x => x + 2]) {
        println!("f(1) = {}", f(1));
    }
}
