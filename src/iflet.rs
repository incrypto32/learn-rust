
pub fn run() {
    let config_max = Some(3u8);
    // match
    match config_max {
        Some(max) => println!("The max is {}", max),
        _ => (),
    }
    // This works the same as previous
    if let Some(max) = config_max {
        println!("The max is {}", max);
    }
}
