use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::vec;

// Macros are code that rights code
// macros are similar to functions, but they are expanded before the compiler interprets the code
// There are two types of macros: declarative and procedural

#[macro_export]
macro_rules! vec_test {
    // One or more comma-separated expressions followed by a comma
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Procecudral macros are functions that accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do.
// The code for this is in hello_macro and hello_macro_derive  crate in the project root
#[derive(HelloMacro)]
struct Pancakes;

pub fn run() {
    let arr = vec![1, 2];
    let arr2 = vec_test![1, 2, 3, 4, 5, 6];
    println!("{:?}", arr);
    println!("{:?}", arr2);


    Pancakes::hello_macro();
}

// There are more types of macros like custom derive macros, attribute macros, and function-like macros
// TODO: Learn about custom derive macros, attribute macros, and function-like macros