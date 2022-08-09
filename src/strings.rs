// There are two types of strings
// Primitive strings which are fixed length and immutable
// String which is a heap allocated data struture - Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello";
    let mut world = String::from("World!");

    world.push('\u{1F600}');
    world.push_str("\nHow you doin!!");
    println!("{}, {}", hello, world);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // Using the format macro to concatenate string without passing ownership
    let s3 = format!("{} {}", s1, s2);
    // Passing the ownership of s1 to s3 and concatenating
    // Here s3 takes the ownership of s1 and we cannot use s1 after it
    let s4 = s1 + &s2;

    println!("{}\n{}", s3, s4);

    // Iterating over chars
    for e in s2.chars() {
        println!("{}", e);
    }

    // Iterating over bytes of string

    for e in s2.bytes() {
        println!("{}", e);
    }
}
