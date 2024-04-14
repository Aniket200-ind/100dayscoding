# Number Guessing Game in Rust

A simple console-based number guessing game implemented in Rust.

## Description

This project is a basic number guessing game where the user tries to guess a random number between 1 and 100. The game provides feedback to the user after each guess, indicating whether the guessed number is too high, too low, or correct. The goal is to guess the correct number with the fewest attempts.

---

## Concepts I learned

- Using crates in Rust.

- Accepting input from the user.

- Handling basic errors.

- Parsing the input to the desired data type.

- Generating random numbers.

- Sleeping the program for a specific time.

---

## Features

- Random number generation using the `rand` crate.
- User input handling and validation.
- Clear feedback on each guess.
- Game loop until the correct number is guessed.

---

## How to Play

1. Clone the repository:

```bash
   git clone https://github.com/your-username/number-guessing-game-rust.git
```

2. Navigate to the project directory:

```bash
   cd number-guessing-game-rust
```

3. Run the game:

```bash
    cargo run
```

---

## Requirements

- Rust (Ensure you have Rust installed, you can install it from here)(https://www.rust-lang.org/)

- Cargo (Ensure you have Cargo installed, it comes with Rust)

- A terminal (cmd, bash, powershell, etc.)

---

## Usage

- The game will prompt you to enter your guess.
- Input a number between 1 and 100 and press Enter.
- Receive feedback on whether your guess is too high, too low, or correct.
- Repeat until you guess the correct number.

---

## Example

```
Guess the lucky number between 1 and 100...
50
The lucky number is greater than you have entered!
75
The lucky number is smaller than you have entered!
60
The lucky number is greater than you have entered!
...
Congratulations! You guessed the number!
You guessed the correct number in 8 guesses.
```

---

## Author

- [Aniket Botre](https://github.com/Aniket200-ind)
