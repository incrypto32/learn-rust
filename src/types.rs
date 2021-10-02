pub fn run() {
    // Default is i32
    let x = 1;

    // Default is float64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 349270234;

    let b: bool = true;

    let c: char = 'n';

    let face: char = '\u{1F600}';
    
    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    println!("Max i64: {}", std::i64::MAX);

    println!("{:?}", (x, y, z, b, c, face))
}
