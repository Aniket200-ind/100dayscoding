fn main() {
    let mut counter = 1;
    let mut iterator = 10;

    // ! Looping with loop statment
    println!("Looping with Loop statement");
    loop {
        println!("{iterator}");
        if iterator == 5 {
            break;
        }
        iterator -= 1;
    }

    // ! While loop
    println!("While loop coming ahead!!");
    while counter <= 10 {
        if counter < 10 {
            print!("{counter}, ");
        } else {
            println!("{counter}");
        }
        counter += 1;
    }

    // ! Looping through arrays
    let arr = [1, 20, 3, 40, 50, 60];
    println!("Looping through arrays using for loop");
    for element in arr {
        print!("{element}, ");
    }

    // ! for loop with range
    println!("\nFor loop with range");
    for i in (1..=10).rev() {
        print!("{i}, ");
    }
}
