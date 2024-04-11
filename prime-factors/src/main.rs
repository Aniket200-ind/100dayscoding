fn main() {
    let mut user_input = String::new();
    println!("Enter a number to find its prime factors: ");
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: u64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };
    let factors = factors(user_input);
    println!("The prime factors of {} are: {:?}", user_input, factors);
}

fn factors(mut n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let mut f = 2;
    while n > 1 {
        while n % f == 0 {
            divisors.push(f);
            n /= f;
        }
        f = if f == 2 { 3 } else { f + 2 }; 
    }
    divisors
}
