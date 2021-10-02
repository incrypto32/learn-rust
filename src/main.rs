use std::io;
use std::io::Write;
mod data_types;
mod functions;
mod guessing_game;
mod ifelse;
mod r#loop;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;

fn main() {
    print!("Enter your Choice : ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    println!("You guessed: {}", choice);

    let choice: u32 = choice.trim().parse().expect("Please type a number!");
    match choice {
        1 => guessing_game::main(),
        2 => variables::main(),
        3 => data_types::main(),
        4 => functions::main(),
        5 => ifelse::main(),
        6 => r#loop::main(),
        7 => structs::main(),
        8 => print::run(),
        9 => types::run(),
        10 => strings::run(),
        11 => tuples::run(),
        _ => (),
    }
}
