pub fn main() {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io::{stdin, stdout, Write};
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        print!("Input your guess : ");
        stdout().flush().unwrap();
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read input");
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
