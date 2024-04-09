fn main() {
    let mut user_input = String::new();
    println!("Enter a year: ");
    std::io::stdin().read_line(&mut user_input).unwrap();
    let year: u64 = user_input.trim().parse().unwrap();
    match year {
        year if is_leap_year(year) => println!("{} is a leap year", year),
        _ => println!("{} is not a leap year", year)
    }
    
}

fn is_leap_year(year: u64) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0 )
}