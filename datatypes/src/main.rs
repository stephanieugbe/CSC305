mod how_you_hold_data_foroperations;
fn main() {
    println!("Hello, world!");
    how_you_hold_data_foroperations::primitives::compound::array_example();
    how_you_hold_data_foroperations::primitives::compound::slice_example();
    how_you_hold_data_foroperations::primitives::compound::tuple_example();
    how_you_hold_data_foroperations::primitives::scalar::get_integer();
    how_you_hold_data_foroperations::primitives::scalar::is_true();
    how_you_hold_data_foroperations::primitives::scalar::floating_point_types();
    how_you_hold_data_foroperations::primitives::scalar::main();
    //how_you_hold_data_foroperations::primitives::scalar::never_type();
    how_you_hold_data_foroperations::primitives::scalar::charact();
    how_you_hold_data_foroperations::derived::user_defined::struct_example();
    how_you_hold_data_foroperations::derived::user_defined::enum_example();


}
