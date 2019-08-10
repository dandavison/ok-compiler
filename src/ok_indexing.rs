pub fn main() {
    let s = String::from("0123456789");
    println!("s[0..7] = {}", &s[0..7]);
    println!("s[7..9] = {}", &s[7..9]);
    let s = String::from("0");
    println!("s[1..] = {}", &s[1..]);
    let s = vec![7];
    println!("s[1..] = {:?}", &s[1..]);

    for i in 2..=2 {
        println!("{}", i);
    }
}
