macro_rules! process_tuples {
    ([($field_11:expr, $field_12:expr), ($field_21:expr, $field_22:expr)]) => {
        println!("field_11 = {}, field_12 = {}", $field_11, $field_12);
        println!("field_21 = {}, field_22 = {}", $field_21, $field_22);
    };
}

pub fn main() {
    process_tuples!([("a11", "a12"), ("a21", "a22")]);
}
