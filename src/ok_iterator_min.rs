struct X {
    a: usize,
}

fn get_minimal_structs() -> Vec<X> {
    let mut selection = Vec::<X>::new();
    let mut candidates = vec![X { a: 1 }, X { a: 2 }];
    selection.push(candidates.drain(..).min_by_key(|x| x.a).unwrap());
    selection
}

pub fn main() {
    println!("{}", get_minimal_structs()[0].a);
}
