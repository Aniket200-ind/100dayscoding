fn main() {
    let num: i32 = 4;

    if num % 2 == 0 {
        println!("{num} is an even number!");
    } else if num < 0 {
        println!("Enter a positive number!");
    } else {
        println!("{num} is a odd number!");
    }

    let age = 20;
    let result = if age > 18 {
        "You can drive"
    } else {
        "You cannot drive!"
    };
    println!("{result}");

    // ! Match statement
    let case_num = 3;

    match case_num {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        4 => println!("The number is 4"),
        _ => println!(" The number is invalid"),
    }

    let time_of_the_day = "morning";

    match time_of_the_day {
        "morning" => println!("Good Morning"),
        "afternoon" => println!("Good Afternoon"),
        "evening" => println!("Good Evening"),
        "night" => println!("Good Night"),
        _ => println!("Enter the correct time"),
    }

    match num {
        0 | 2 | 4 | 6 | 8 | 10 => println!("{num} is even**!"),
        1 | 3 | 5 | 7 | 9 => println!("{num} is odd**!"),
        _ => println!("Number should be between 0 and 10!!"),
    }

    match age {
        4..=15 => println!("Child"),
        16..=20 => println!("Teenage"),
        21..=30 => println!("Adult"),
        _ => println!("Invalid age"),
    }
}
