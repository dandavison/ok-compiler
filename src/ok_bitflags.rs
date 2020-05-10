use bitflags::bitflags;

bitflags! {
    struct Choice: u8 {
        const EMPTY = 0;
        const OREGANO = 1;
        const ARTICHOKEHEARTS = 2;
        const OLIVES = 4;
        const CAPERS = 8;
    }
}

pub fn main() {
    let mut pizza = Choice::EMPTY;
    pizza |= Choice::ARTICHOKEHEARTS;
    pizza |= Choice::CAPERS;

    println!("Pizza is {:?}", pizza);
    println!(
        "Contains OREGANO (should be false): {:?}",
        pizza & Choice::OREGANO
    );
    println!(
        "Contains ArtichokeHearts (should be true): {:?}",
        pizza & Choice::ARTICHOKEHEARTS
    );
    println!(
        "Contains Olives (should be false): {:?}",
        pizza & Choice::OLIVES
    );
    println!(
        "Contains Capers (should be true): {:?}",
        pizza & Choice::CAPERS
    );
}
