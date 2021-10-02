pub fn main() {


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // Returns value from a loop
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    // Conditional Loops

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // Loop over items in a list
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


    // PAVARRRRRR
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
