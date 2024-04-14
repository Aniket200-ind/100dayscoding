# Error Handling in Rust

> [Error Handling Rust documentation](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

- Rust groups errors into two major categories: recoverable and unrecoverable errors.

- For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation.

- Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

- Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

## Unrecoverable Errors with panic!

- The panic! macro can be used to generate a panic when certain conditions occur.

- panic! is often used when a program reaches an unrecoverable state because of a bug.

It occurs when:

- An assertion fails

- Calling panic! explicitly

- When a program tries to access an element of an array or vector at an index that does not exist

## Recoverable Errors with Result

- Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

- The Result enum is defined as having two variants, Ok and Err, as follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- The T and E are generic type parameters. The Ok variant indicates that the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

- By using Result, you can express all the error-handling possibilities your functions might encounter so there’s a better chance that you’ll handle them appropriately. This means when you see a Result value, you can tell immediately what kind of error handling is happening.

```rust
let result = eligible(13);
    match result {
        Ok(age) => {
            println!("Person eligible to vote with age={}", age);
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }

fn eligible(age: u32) -> Result<u32, String> {
    if age < 18 {
        return Err(String::from("Person not eligible to vote"));
    }
    Ok(age)
}
```

## Options

- The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.

- Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there. In languages with null, variables always hold a value of either some data or null. In this way, null is similar to None in Rust.

```rust
let fruits = vec!["mango", "strawberry", "apple", "banana", "orange", "guava"];

    for &index in [0, 1, 2, 3, 4, 5, 10].iter() {
        match fruits.get(index) {
            Some(&"strawberry") => println!("I like strawberries!!"),
            Some(fruit_name) => println!("It's a delicious {}", fruit_name),
            None => println!("Sorry, Fruit not availabe!"),
        }
    }
```
