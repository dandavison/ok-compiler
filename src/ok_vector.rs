fn f1() {
    let mut vec_of_strings: Vec<String> = Vec::new();
    vec_of_strings.push(String::from("Pablo"));
    println!("{:?}", vec_of_strings);
}

fn f2() {
    let mut array_of_mut_vecs_of_strings: [Vec<String>; 2] = [Vec::new(), Vec::new()];
    array_of_mut_vecs_of_strings[0].push(String::from("Honey"));
    println!("{:?}", array_of_mut_vecs_of_strings);
}

pub fn main() {
    f1();
    f2();
}
