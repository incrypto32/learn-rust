
pub fn run() {
    let num_list = vec![4, 2, 3, 10, 6, 4];
    let char_list = vec!['a', 'b', 'k', 'c'];
    println!("Largest number is {}", get_largest(num_list));
    println!("Largest car is {}", get_largest(char_list));

    let _p1 = Point { x: 1, y: 2 };
    let _p2 = Point { x: 10.1, y: 8.2 };
    _p1.x;
    _p1.y;
}

// PatialOrd + Copy are traits for the type T (Comparison and copy)
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}
