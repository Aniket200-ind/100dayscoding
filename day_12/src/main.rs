struct Rectangle {
    width: u16,
    height: u16,
}

// ! Keep associated methods and methods in different `imp` block for better readability and understanding
impl Rectangle {
    fn square (size: u16) -> Self {
        Self { width: size, height: size }
    }
}

impl Rectangle {
    fn get_width(&self) -> u16 {
        self.width
    }
    fn get_height(&self) -> u16 {
        self.height
    }
    fn area(&self) -> u16 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 10,
    };

    let area = rect1.area();
    let width = rect1.get_width();
    let height = rect1.get_height();

    let square = Rectangle::square(30);

    println!("The area of the rectangle with height: {height} and width: {width} is {area}");

    println!("The width and height of the square is {} and {} respectively.", square.width, square.height);
}
