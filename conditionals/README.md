# Conditionals

## If Statements

As other programming languages Rust also supports the following conditionals:

- If statements
- If-else statements
- If-else if statements

```rust
 let num: i32 = 4;

    if num % 2 == 0 {
        println!("{num} is an even number!");
    }else if num < 0 {
        println!("Enter a positive number!");
    }
    else {
        println!("{num} is a odd number!");
    }
    // Output: 4 is an even number!
```

All `if`expressions start with the keyword `if`, followed by a condition.

In the above example, the condition first checks whether the number is even or not.

- If the condition is true, then the code inside the curly braces `{}` is executed.
- If the condition is false, then the code of if block is skipped.
- The `else` block is optional.
- If the condition of `if` block is false, then the condition of `else if` block is checked.
- If it is true, then the code inside the curly braces `{}` is executed.
- If the condition is false, then the code of `else if` block is skipped.
- The `else` block is optional. If the condition of `else if` block is false, then the code inside the curly braces `{}` is executed.

> Note: We can also include the if...else condition inside the let statements.

```rust
let age = 20;
let result = if age > 18 {
        "You can drive"
    } else {
        "You cannot drive!"
    };
    println!("{result}");
    // Output: You can drive
```

The `result` variable will be bound to a value based on the outcome of the if expression.

## Match control flow construct

> Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. That's where match comes into play!!

The `match` keyword is used to compare a value against a series of patterns and then execute code based on which pattern matches.

Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into.

(Source: https://doc.rust-lang.org/book/ch06-02-match.html)

```rust
let num = 3;

    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
    // Output: Three
```

Rust saves our effort to write break statements after each case as only the case corresponding
to the match is executed. Default case, where no match is found is represented by `_` (underscore).

> Note: The match expression must be exhaustive, which means that all possible values must be matched.

### We can have multiple values in a single case

```rust
let num: i32 = 4;

match num {
        0 | 2 | 4 | 6 | 8 | 10 => println!("{num} is even**!"),
        1 | 3 | 5 | 7 | 9 => println!("{num} is odd**!"),
        _ => println!("Number should be between 0 and 10!!"),
    }
    // Output: 4 is even**!
```

### We can also use ranges in match expressions

```rust
let age = 20;

match age {
        4..=15 => println!("Child"),
        16..=20 => println!("Teenage"),
        21..=30 => println!("Adult"),
        _ => println!("Invalid age"),
    }
    // Output: Teenage
```

