// Option enum represents a Value or No Value at all
// Many functions in rust return this to indicate a sort of nullable value as in other languages

pub fn run() {
    let name = String::from("Bhadra");
    // When we try to get a value at index 8 (which does not exist) we need a way to represent the absemse of value
    // Option have two possible value. A Some() or None
    println!(
        "Character at index n is {}",
        match name.chars().nth(1) {
            Some(c) => c.to_string(),
            None => "No character at index 8".to_string(),
        }
    );

    print_address("Bhadra");
    print_address("Sasi");
}

fn address(name: &str) -> Option<&str> {
    match name {
        "Bhadra" => Some("Calicut"),
        "Manu" => Some("Kochi"),
        "Krishnanand" => Some("Thrissur"),
        _ => None,
    }
}

fn print_address(name: &str) {
    match address(name) {
        Some(address) => println!("{} lives in {}", name, address),
        None => println!("{} does not live in any city", name),
    }
}
