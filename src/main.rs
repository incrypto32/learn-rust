use std::io;
use std::io::Write;

mod arrays;
mod collections;
mod data_types;
mod enums;
mod functions;
mod guessing_game;
mod hashmaps;
mod ifelse;
mod r#loop;
mod option;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;
mod panic;
mod error_handling_result_enum;

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
        15 => ifelse::run(),
        16 => collections::run(),
        17 => hashmaps::run(),
        18 => panic::run(),
        19 => error_handling_result_enum::run(),
        _ => (),
    }
}
