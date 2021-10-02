pub fn run() {
    // Basic formatting
    println!("{} is from {}. He is {} years old", "Kichu", "Thrissur", 21);

    // Positional arguments
    println!(
        "{0} is first then {1} then again {0} and then {2}",
        "A", "B", "C"
    );

    // Named arguments
    println!("My name is {name} and I'm {age}", name = "Kichu", age = 21);

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"))
}
