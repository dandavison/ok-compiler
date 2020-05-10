use bit_set::BitSet;

enum Choice {
    Oregano = 1,
    ArtichokeHearts = 2,
    Olives = 4,
    Capers = 8,
}

pub fn main() {
    let mut pizza = BitSet::new();
    pizza.insert(Choice::ArtichokeHearts as usize);
    pizza.insert(Choice::Capers as usize);

    println!(
        "Contains Oregano (should be false): {}",
        pizza.contains(Choice::Oregano as usize)
    );
    println!(
        "Contains ArtichokeHearts (should be true): {}",
        pizza.contains(Choice::ArtichokeHearts as usize)
    );
    println!(
        "Contains Olives (should be false): {}",
        pizza.contains(Choice::Olives as usize)
    );
    println!(
        "Contains Capers (should be true): {}",
        pizza.contains(Choice::Capers as usize)
    );
}
