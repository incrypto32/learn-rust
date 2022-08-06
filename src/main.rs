use std::io;
use std::io::Write;
mod arrays;
mod data_types;
mod functions;
mod guessing_game;
mod ifelse;
mod r#loop;
mod option;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;
mod enums;

fn main() {
    print!("Enter your Choice : ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    println!("Your choice: {}", choice);

    let choice: u32 = choice.trim().parse().expect("Please type a number!");
    match choice {
        1 => guessing_game::run(),
        2 => variables::run(),
        3 => data_types::run(),
        4 => functions::run(),
        5 => ifelse::run(),
        6 => r#loop::run(),
        7 => structs::run(),
        8 => print::run(),
        9 => types::run(),
        10 => strings::run(),
        11 => tuples::run(),
        12 => arrays::run(),
        13 => option::run(),
        14 => enums::run(),
        _ => (),
    }
}
