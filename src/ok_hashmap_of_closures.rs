use std::collections::HashMap;

type BoxedClosure = Box<dyn Fn() -> bool>;

fn get_key_value_pairs() -> Vec<(String, BoxedClosure)> {
    vec![("key_1".to_string(), Box::new(|| true))]
}

fn get_key_value_pairs_2() -> Vec<(String, BoxedClosure)> {
    let mut pairs = get_key_value_pairs();
    pairs.extend(vec![("key_2".to_string(), Box::new(|| false) as BoxedClosure)].into_iter());
    pairs
}

fn get_pairs() -> Vec<(bool, bool)> {
    vec![(false, false)]
}

fn get_pairs_2() -> Vec<(bool, bool)> {
    let mut pairs = get_pairs();
    pairs.extend([(false, true)].iter());
    pairs
}

fn make_hashmap() -> HashMap<String, BoxedClosure> {
    get_key_value_pairs().into_iter().collect() // .into_iter(), not .iter()
}

fn make_hashmap_2() -> HashMap<String, BoxedClosure> {
    let mut hashmap = HashMap::<String, BoxedClosure>::new();
    for (k, f) in get_key_value_pairs() {
        hashmap.insert(k, f);
    }
    hashmap
}

pub fn main() {
    let hashmap = make_hashmap();
    for (k, f) in &hashmap {
        println!("{} => {}", k, f());
    }
    let hashmap = make_hashmap_2();
    for (k, f) in &hashmap {
        println!("{} => {}", k, f());
    }
}
