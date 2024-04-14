use std::io;

fn factorial(&num: &u128) -> u128 {
    if num == 1 {
        return 1;
    }

    num * factorial(&(num - 1))
}

fn main() {
    let mut user_input = String::new();
    let num: u128;

    println!("Enter a number: ");
    loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input, please try again");

        num = match user_input.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("Please enter  a valid number!!");
                continue;
            }
        };
        break;
    }

    let result = factorial(&num);
    println!("Factorial of {} is {}", num, result);
}
