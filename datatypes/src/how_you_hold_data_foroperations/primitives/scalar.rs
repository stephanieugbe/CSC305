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
pub fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
//pub fn never_type()-> ! {
  //      panic!("This call never returns.");
    
//}
    