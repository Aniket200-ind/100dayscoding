#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}


struct Coordinates {
    x: u8,
    y: u8,
    direction: Direction,
}


enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Traingle(f64, f64, f64),
}

impl Shape{
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 3.14 * radius * radius,
            Shape::Rectangle(length, width) => length * width,
            Shape::Traingle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    let direction1 = Direction::North;
    let direction2 = Direction::East;
    let direction3 = Direction::West;
    let direction4 = Direction::South;
    directions(&direction1);
    directions(&direction2);
    directions(&direction3);
    directions(&direction4);

    let coordinates = Coordinates {
        x: 10,
        y: 20,
        direction: Direction::North,
    };

    println!("The coordinates of specified location are {}, {}, in the {:?} direction", coordinates.x, coordinates.y, coordinates.direction);

    let rectange = Shape::Rectangle(12.4, 21.8);
    let area = rectange.area();
    println!("Area of the rectangle is {}", area);
}

fn directions(direction: &Direction){
    match direction {
        Direction::North => println!("I'm heading north!"),
        Direction::South => println!("I'm heading south!"),
        Direction::East => println!("I'm heading east!"),
        Direction::West => println!("I'm heading west!"),
    }
}
