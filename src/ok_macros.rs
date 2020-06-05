#[derive(Default, Debug)]
struct S<'a> {
    field_1: &'a str,
    field_2: &'a str,
}

macro_rules! set_fields {
    ([$(($field_name:ident, $val:expr)),* ], $the_struct:expr) => {
        $(
            $the_struct.$field_name = $val;
        )*
    };
}

pub fn main() {
    let mut s = S::default();
    set_fields!([(field_1, "val_1"), (field_2, "val_2")], &mut s);
    println!("{:?}", s);
}
