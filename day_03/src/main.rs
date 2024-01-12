fn main() {
    // ! Creating and initializing a variable
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; --> This line will produce an error

    // ! Mutating variables
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 22;
    println!("The value of y is: {}", y);

    // ! Const

    let tot_hours = 24;
    const MINUTES_IN_HOUR: i32 = 60;
    println!(
        "Total minutes in a day are: {}",
        MINUTES_IN_HOUR * tot_hours
    );

    // ! Shadowing
    let a = 23;
    let a = a - 3;
    {
        let a = a / 2;
        println!("The value of a in the scope is: {}", a);
    }
    println!("The value of a outside the scope is: {}", a);

    // ? We can change the data type while shadowing
    let str: &str = "Hello World!"; // * We will cover strings later...
    let str = str.len();
    println!("The number of characters in the string is: {}", str);
}
