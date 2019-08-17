#[derive(Clone)]
struct X {
    a: usize,
}

fn get_minimal_structs() -> Vec<X> {
    let mut selection = Vec::<X>::new();
    let candidates = [X { a: 1 }, X { a: 2 }];
    selection.push(candidates.iter().min_by_key(|x| x.a).unwrap().clone());
    selection
}

pub fn main() {
    println!("{}", get_minimal_structs()[0].a);
}
