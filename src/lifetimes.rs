pub fn run() {
    // let r;

    // {
    //     let x = 5;
    // r = &x;  // error here x will not live long enough by borrow checker
    // }
    // println!("r : {}", r)

    // This kind of lifetime is particularly tricky for functions returning reference
    //

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

// 'a is a generic lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
