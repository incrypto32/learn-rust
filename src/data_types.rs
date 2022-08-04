pub fn run() {
    // Parsing a string to u32 you need to specify the type here
    //  If type isnt specified compiler wont know which data type to parse into
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // Floating point numbers

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("{} {}", x, y);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // rerunder
    let rerunder = 43 % 5;

    println!(
        "sum : {}\n difference : {}\n product:{}\n quotient:{}\n rerunder:{}\n",
        sum, difference, product, quotient, rerunder
    );

    // The boolean type
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("{} {}", t, f);

    // Character Type

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{:?}", tup);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("Indexing tuple {} {} {} ", tup.0, tup.1, tup.2);

    // Array type

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    
    println!("The array is {:?}",months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array is {:?}",a);
    
    //  Accessing array elements
    println!("{}", a[0])

}
