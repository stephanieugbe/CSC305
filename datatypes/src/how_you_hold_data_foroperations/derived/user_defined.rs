
struct Person {
    name: String,
    age: u32,
}

pub fn struct_example(){
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
};
println!("Name: {}", alice.name);
    println!("Age: {}", alice.age);
}
enum Color {
    Red,
    Green,
    Blue,
}
pub fn enum_example(){
    let favorite_color = Color::Blue;
    let secondary_color = Color::Green;

    // Match on the 'favorite_color' and 'secondary_color' enums
    match favorite_color {
        Color::Red => println!("Red is your favorite color!"),
        Color::Green => println!("Green is your favorite color!"),
        Color::Blue => println!("Blue is your favorite color!"),
    }

    match secondary_color {
        Color::Red => println!("Red is your secondary color."),
        Color::Green => println!("Green is your secondary color."),
        Color::Blue => println!("Blue is your secondary color."),
    }
}

