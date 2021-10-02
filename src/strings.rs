// There are two types of strings
// Primitive strings which are fixed length and immutable
// String which is a heap allocated data struture - Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello";
    let mut world = String::from("World!");

    world.push('\u{1F600}');
    world.push_str("\nHow you doin!!");
    println!("{}, {}", hello, world);
}
