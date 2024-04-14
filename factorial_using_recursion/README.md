# Finding factorial using recursion

Today for Day 19 of #100DaysOfCode, I decided to try the factorial problem using recursion in Rust. I have done this in other languages before, but I wanted to see how it would work in Rust.

I started by creating a function that would take in a number and return the factorial of that number. I then created a main function that would call the factorial function and print the result. I then ran the program and it worked as expected🥳.


```rust
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
```

I also took a step forward and decided to accept the user input from the command line. I did this by creating a loop that would keep asking the user for input until they entered a valid number. I then parsed the input to a number and passed it to the factorial function. I then printed the result.


## Lessons Learned

- Recursion is a very powerful tool in programming. It can be used to solve a lot of problems.

- Rust is a very powerful language. I am really enjoying learning it.

- I guess I have also started to think in Rust way. Like while designing the function, I was thinking about how to pass the variable by reference and not by value. I guess this is a good thing, as it will help me write better code in Rust and make memory efficient programs.
