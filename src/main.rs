mod ok_cansi;
mod ok_enum;
mod ok_generic;
mod ok_indexing;
mod ok_iterator;
mod ok_map;
mod ok_reference;
mod ok_struct;
mod ok_struct_setter;
mod ok_struct_wrapper;
mod ok_syntect;
mod ok_vector;
mod ok_string;

fn main() {
    ok_struct::main();
    ok_reference::main();
    ok_generic::main();
    ok_vector::main();
    ok_syntect::main();
    ok_indexing::main();
    ok_struct_setter::main();
    ok_struct_wrapper::main();
    ok_cansi::main();
    ok_iterator::main();
    ok_enum::main();
    ok_map::main();
    ok_string::main();
}
