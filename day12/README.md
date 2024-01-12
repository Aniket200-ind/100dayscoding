# Methods and `impl` keyword

Methods are functions attached to objects. These methods have access to the data of the object and its other methods via the `self` keyword. Methods are defined under an `impl` block.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // The area of the rectangle is 1500 square pixels.
}
```

The first parameter of a method is always `self`, which represents the instance of the struct the method is being called on. The `self` parameter is passed automatically, and we don’t need to specify it when we call the method.


There are several different ways to specify parameters of self methods:

- `self` — meaning the method takes ownership of the instance it is called on.

- `&self` — meaning the method takes an immutable reference to the instance via the `self` keyword.

- `mut self` - meaning the method mutates the instance it is called on.

- `&mut self` — meaning the method takes a mutable reference to the instance via the `self` keyword.

---

## Associated functions

Associated functions are functions associated with a struct. They don't take `self` as a parameter. They are often used for constructors that will return a new instance of the struct. They are like static methods in other languages.

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    
    println!("The area of the square is {} square pixels.", sq.area());
    // The area of the square is 9 square pixels.
}
```

In the example above, we create a square associated function that takes a size parameter and returns a new instance of the Rectangle struct with the same width and height.

> **Note:** We can keep multiple methods in single implementation block. But it is recommended to keep related methods in the same implementation block. For example, we can keep all the associated methods in a single implementation block and all the methods that take `self` as a parameter in another implementation block.
