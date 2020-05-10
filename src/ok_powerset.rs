#[derive(Copy, Clone, Debug)]
enum Choice {
    Oregano = 1,
    ArtichokeHearts = 2,
    Olives = 4,
    Capers = 8,
}

#[derive(Copy, Clone, Debug)]
struct Powerset {
    bit_vector: u8,
}

impl Powerset {
    pub fn new() -> Self {
        Self { bit_vector: 0 }
    }

    pub fn add(&mut self, choice: Choice) {
        self.bit_vector |= choice as u8;
    }

    pub fn contains(&self, choice: Choice) -> bool {
        self.bit_vector & choice as u8 > 0
    }
}

pub fn main() {
    let mut pizza = Powerset::new();
    dbg!(pizza);
    pizza.add(Choice::ArtichokeHearts);
    dbg!(pizza);
    pizza.add(Choice::Capers);
    dbg!(pizza);

    println!(
        "Contains Oregano (should be false): {}",
        pizza.contains(Choice::Oregano)
    );
    println!(
        "Contains ArtichokeHearts (should be true): {}",
        pizza.contains(Choice::ArtichokeHearts)
    );
    println!(
        "Contains Olives (should be false): {}",
        pizza.contains(Choice::Olives)
    );
    println!(
        "Contains Capers (should be true): {}",
        pizza.contains(Choice::Capers)
    );
}
