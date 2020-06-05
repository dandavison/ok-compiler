macro_rules! process_tuples {
    ([($field_1:expr, $field_2:expr)]) => {
        println!("field_1 = {}, field_2 = {}", $field_1, $field_2);
    };
}

pub fn main() {
    process_tuples!([("a11", "a12")]);
}
