use std::collections::HashMap;

pub fn run() {
    let blue = String::from("Blue");
    let yellow = String::from("Yello");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    scores.entry(String::from("Red")).or_insert(30); // If the entry does'nt exist create it
    scores.entry(String::from("Red")).or_insert(50); // Will not be inserted.

    println!("{:?}", scores.get(&String::from("Red")));

    // Counting the number of times of a word

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    // Looping through the list of words
    // And map.entry returns pointer to the entry
    // Can modify the value directly using * operator
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
