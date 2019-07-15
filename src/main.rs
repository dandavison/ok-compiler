mod ok_generic;
mod ok_indexing;
mod ok_reference;
mod ok_struct;
mod ok_struct_setter;
mod ok_syntect;
mod ok_vector;


fn main() {
    ok_struct::main();
    ok_reference::main();
    ok_generic::main();
    ok_vector::main();
    ok_syntect::main();
    ok_indexing::main();
    ok_struct_setter::main();
}
