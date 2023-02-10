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

    struct_with_lifetime()
}

// 'a is a generic lifetime annotation
// this says that the reference returned by the function will live as long as the
// shorter of the lifetimes of the references passed in
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Here the struct lives only for the life time of the referenece part
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// method lifetimes
// Here the method return_part will live only for the life time of the referenece part
impl<'a> ImportantExcerpt<'a> {

    // Here the lifetime is inferred by the compiler
    fn return_part(&self, announcement: &str) ->  &str {
        println!("Announcement : {}", announcement);
        return self.part;
    }
}

fn struct_with_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i);
    println!("{:?}", i.return_part("Hello"));
}
