#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// These are tuple structs without any named fields.
struct Point2D(u32, u32);
struct Circle(u32);

// Unit Structs are structs without any fields.
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
struct Unit;

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };
    let rect3 = Rectangle::square(40);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Creating Instances From Other Instances With Struct Update Syntax
    let rect4 = Rectangle { width: 30, ..rect3 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!(
        "The area of the rectangle 4 is {} square pixels.",
        rect4.area()
    );

    let p1 = Point2D(10, 20);
    let c = Circle(10);
    println!("p1 is {},{}", p1.0, p1.1);
    println!("c is {}", c.0);

    let unit = Unit;
    println!(
        "unit is {}",
        match unit {
            Unit => "Unit",
        }
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
