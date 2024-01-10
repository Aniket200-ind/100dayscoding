use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time::Duration;

fn main() {
    let random_number: u8 = rand::thread_rng().gen_range(1..=100);
    let mut no_of_guesses: u8 = 0;

    loop {
        let mut user_input = String::new();
        no_of_guesses += 1;

        println!("Guess the lucky number between 1 and 100...");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the input, please try again");

        let user_input: u8 = match user_input.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => num,
            _ => {
                println!("Please enter a valid number between 1 and 100!");
                continue;
            }
        };

        match user_input.cmp(&random_number) {
            Ordering::Less => println!("The lucky number is greater than you have entered!"),
            Ordering::Greater => println!("The lucky number is smaller than you have entered!"),
            Ordering::Equal => {
                println!("The lucky number is {}", random_number);
                println!(
                    "You guessed the correct number in {} guesses",
                    no_of_guesses
                );
                break;
            }
        }
    }
    std::thread::sleep(Duration::from_secs(2));
}
