
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("The area of the shape is {:.2}", shape.area());
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 4.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}