pub fn tuple_example() {
    let person = ("Alice", 30, 5.8);

    println!("Tuple:");
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}", person.2);
}

pub fn array_example() {
    let numbers = [1, 2, 3, 4, 5];

    println!("Array:");
    for num in &numbers {
        println!("{}", num);
    }
}
pub fn slice_example(){
let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
let _slice: &[i32] = &boxed_array[..];
}