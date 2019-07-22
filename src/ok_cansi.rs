use cansi;

pub fn main() {
    let text = "Hello \x1b[31mWorld\x1b[0m";
    let parsed = cansi::categorise_text(text);
    println!("{:?}", parsed[0].fg_colour);
}
