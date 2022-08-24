use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more..") // Default implmentation
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Bhadra is cute"),
        content: String::from("The famous Bhadra moves to EKM"),
    };

    println!("{}", tweet.summarize());
    println!("{}", article.summarize());
    notify(&tweet);
}

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// // Here is a longer version of this
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn _notify2(_item1: &(impl Summary + Display), _item2: &impl Summary) {}

pub fn _notify3<T: Summary + Display>(_item1: &T, _item2: &T) {}

fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}

// A More readable version of above
fn _some_function2<T, U>(_t: &T, _u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// Implement a trait on a another trait
impl<T: Display> Summary for T {
}
