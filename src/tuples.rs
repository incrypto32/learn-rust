pub fn run() {
    let person: (&str, &str, i32) = ("Kavu", "Suresh", 20);

    println!("{} {} {}", person.0, person.1, person.2);
}
