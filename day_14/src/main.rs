use std::f64::consts::PI;
use std::ops::Add;

struct Point<T, U> {
    x: T,
    y: U,
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    length: f64,
    breadth: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.breadth
    }
}

fn main() {
    let boolean = Point { x: true, y: "low" };
    let integer = Point { x: 10, y: 30.2 };
    let float = Point { x: 5.4, y: 20 };
    let str = Point {
        x: "high",
        y: false,
    };

    println!("Addition between 10 & 20 is {}", sum_generic(10, 20));
    println!(
        "Addition between 10.1 & 20.3 is {}",
        sum_generic(10.1, 20.3)
    );

    let circle = Circle { radius: 10.3 };
    let rectangle = Rectangle {
        length: 10.2,
        breadth: 21.3,
    };

    println!("Area of the circle is {}", circle.area());
    println!("Area of the rectangle is {}", rectangle.area());
}

// generic function that add two numbers
fn sum_generic<T: Add<Output = T> + Copy>(a: T, b: T) -> T {
    return a + b;
}
