use std::fs::{self, File};
use std::io::Read;
use std::io::{self, ErrorKind};

pub fn run() {
    let f = File::open("a.txt");

    let _f = match f {
        Ok(fc) => {
            println!("File Found!!");
            fc
        }
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

    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file : {:?}", error);
            })
        } else {
            panic!("Problem creating the file : {:?}", error);
        }
    });

    // Just panics when file is not available
    let _f = File::open("c.txt").unwrap();
    // Same with custom message
    let _f = File::open("c.txt").expect("Failed to open");

    read_username_from_file().unwrap();
    read_username_from_file_2().unwrap();
    read_username_from_file_3().unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // The ? is similar to the code below

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    Ok(s)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("abcd.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("abcd.txt")
}
