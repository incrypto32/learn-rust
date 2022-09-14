// A closure is an anonymous function that can be created on the go

use std::{thread, time::Duration};

pub fn run() {}

struct  Cacher{
    
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calcualting slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("random_number : {}", random_number);
    if intensity < 25 {
        println!("Do {} pushups today", expensive_closure(intensity));

        println!("Do {} situps today", expensive_closure(intensity));
    } else if intensity < 50 {
        println!("Do {} pushups today", expensive_closure(intensity));
    }
}
