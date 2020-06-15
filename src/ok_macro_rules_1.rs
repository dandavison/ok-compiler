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
    // Additional rule to handle trailing comma after last tuple inside [...]
    ([$(($field_name:ident, $val:expr)),* ,], $the_struct:expr) => {
        set_fields!([$( ($field_name, $val) ),*], $the_struct)
    };
}

fn f() {
    let mut s = S::default();
    set_fields!([(field_1, "val_11"), (field_2, "val_21")], &mut s);
    println!("{:?}", s);
    set_fields!([(field_1, "val_12"), (field_2, "val_22"),], &mut s);
    println!("{:?}", s);
}

pub fn main() {
    f();
}
