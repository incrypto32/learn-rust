// Collections allow you to store multiple values
// Unlike array or heap the size of the collection is not limited

pub fn run() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];

    // Acessing elements in vector
    let third = &v[2];
    println!("The third element is {}", third);
}
