pub fn run() {
    // mut keyword is so that the varibale is mutable
    // otherwise the variable cannot be updated
    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("x is {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("x is {}", MAX_POINTS);

    let y = 5;
    println!("y is {}", y);

    // This process of redeclaring a variable is called shadowing
    let y = y + 1;
    println!("y is {}", y);

    let y = y - 2;
    println!("y is {}", y);

    // Assign multiple variables
    let (a, b) = ("Krishnanand", "V P");
    println!("{} {}", a, b)
}
