pub fn run() {

    // Expressions can return value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    another_function(5, 6);
    addition(2, 3);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn addition(x: i32, y: i32) -> i32 {
    return x + y;
}
