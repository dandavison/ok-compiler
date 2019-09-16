fn use_alphabet(alphabet: &[char]) {
    println!("{:?}", alphabet[1]);
}

pub fn main() {
    let s = "aaa  \n  ";
    println!("__{}__", s);
    println!("__{}__", s.trim_end());
    println!("{:?}", s.chars().nth(1));
    let alphabet: [char; 3] = ['a', 'b', 'c'];
    println!("{:?}", alphabet[1]);
    use_alphabet(&alphabet);
}
