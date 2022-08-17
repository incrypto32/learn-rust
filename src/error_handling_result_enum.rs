use std::{fs::File, io::ErrorKind};

pub fn run() {
    let f = File::open("a.txt");

    let f = match f {
        Ok(fc) => {
            println!("File Found!!");
            fc
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("a.txt") {
                Ok(fc) => {
                    println!("File created");
                    fc
                }
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };
}
