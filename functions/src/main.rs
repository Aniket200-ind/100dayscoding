fn main() {
    const NUM1: i32 = 40;
    const NUM2: i32 = 50;

    greet();
    println!("The result of addition is {}", add(NUM1, NUM2));
    println!("The result of subtraction is {}", subtract(NUM1, NUM2));
    println!("The result of multiplication is {}", multiply(NUM1, NUM2));
    println!("The result of division is {}", divide(NUM1, NUM2));
}

fn greet () {
    println!("Good morning!");
}

fn add (a:i32, b:i32) -> i32 { 
    a + b
}

fn subtract (a:i32, b:i32) -> i32 { 
    a - b
}
fn multiply (a:i32, b:i32) -> i32 { 
    a * b
}
fn divide (a:i32, b:i32) -> i32 { 
    b / a
}