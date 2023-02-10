// A closure is an anonymous function that can be created on the go

use std::{thread, time::Duration};

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        let cacher = Cacher {
            calculation: calculation,
            value: None,
        };

        return cacher;
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calcualting slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cached_result = Cacher::new(expensive_closure);
    println!("random_number : {}", random_number);
    if intensity < 25 {
        println!("Do {} pushups today", cached_result.value(intensity));

        println!("Do {} situps today", cached_result.value(intensity));
    } else if intensity < 50 {
        println!("Do {} pushups today", cached_result.value(intensity));
    }
}
