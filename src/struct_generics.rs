pub fn run() {
    let p1 = Point { x: 1, y: 2 };
    println!("x : {}, y : {}", p1.x(), p1.y);

    let p1 = Point{x:1.2,y:1.3};
    println!("x : {}, y : {}", p1.x(), p1.y());

    let p2 = Point2 { x: 2, y: 10.2 };
    p2.mixup(Point2 { x: 10.1, y: 11.2 });
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, V> {
    x: T,
    y: V,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

// Implemented only for f64 point
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}
