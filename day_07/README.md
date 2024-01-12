# Functions

In Rust we use `fn` keyword to declare a function. The function name is followed by a set of parentheses `()` and the function body is enclosed in curly braces `{}`. The function is called with the function name followed by a set of parentheses `()`.

We have already seen the most used function i.e. the **main** function.

> The main function is the entry point of every program. It is called when the program is executed.

- Example of a simple function

```rust
fn main() {
    greet();
}

fn greet() {
    println!("Greetings Rustacean!!");
}
```

### Some important points to note

- In Rust, the function name should be in `snake_case` i.e. all lowercase letters with underscores between words.

- The function name should be descriptive and should convey the purpose of the function. _(For better readability)_

- The function name should not start with a number.

- In order to execute the function we need to call it from the main function.

---

## Function with parameters

- Example

```rust
fn main() {
    greet("John Doe");
}

fn greet(name: &str) {
    println!("Greetings {name}!!");
}
```

> Note: We need to explicitly mention the type of the parameter.

---

## Function with return value

- Example

```rust
fn main() {
    let result = add(10, 20);
    println!("The sum is {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

> Note: We need to explicitly mention the return type of the function after an arrow `->`.
