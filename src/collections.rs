// Collections allow you to store multiple values
// Unlike array or heap the size of the collection is not limited

pub fn run() {
    let _a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let _v2 = vec![1, 2, 3];

    // Acessing elements in vector

    // Directly. This approach can cause index out of bound error
    let third = &v[2];
    println!("The third element is {}", third);

    // Indirectly using vec.get()
    match v.get(2) {
        Some(c) => println!("The third element is {}", c),
        None => println!("There is no value at the given index"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("{}", i);
    }
}
