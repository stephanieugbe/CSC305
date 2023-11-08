pub fn is_true() -> bool {
    let b: bool = true;
    return b;
}

pub fn get_integer() -> i64 {
    let a: i64 = 8; 
    return a;
}

pub fn floating_point_types() {
    let a: f32 = 7.54;
    let b: f64 = 9.76;

    println!("f32: {}", a);
    println!("f64: {}", b);
}

pub fn charact() {
    let my_char: char = 'A';

    println!("Character:");
    println!("char: {}", my_char);
}

//pub fn never_type()-> ! {
  //      panic!("This call never returns.");
    
//}
    