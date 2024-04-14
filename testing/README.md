# Writing automated tests

Rust is a great language for writing tests. It has a powerful type system, provides useful abstractions for writing tests, and produces fast and reliable test suites.

In this module we will cover unit tests in Rust.

## Unit tests

Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. Unit tests are usually located in the same files as the code they are testing, usually in a `tests` module at the end of the file. The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the tests only when you run `cargo test`, not when you run `cargo build`.

```rust
// src/lib.rs

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

The `#[test]` annotation on the `it_works` function indicates that this is a test function, so the test runner knows to treat this function as a test. The `assert_eq!` macro, provided by the standard library, tests whether two values are equal. If they are not, the macro will call the `panic!` macro, which causes the test to fail. If they are equal, the test will pass.

To run the tests, run `cargo test`


### `#[should_panic]`

Sometimes you might expect your code to fail under certain circumstances. For example, the `add_two` function we defined earlier might only work on positive integers. We can add a test that asserts that the function panics when given a negative number. This test should pass if the function panics as expected and fail if the function does not panic or panics for a different reason.

```rust
// src/lib.rs

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_works() {
        assert_eq!(4, add_two(-2));
    }
}
```

### `#[ignore]`

Sometimes you might want to write tests for functions that are not yet implemented. Annotating a test function with `#[ignore]` will tell the test runner to ignore this test. This is useful when you are writing tests for a new feature and want to write tests for the public API before you have implemented the underlying functionality.

```rust
// src/lib.rs

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(4, add_two(-2));
    }
}
```

### `#[test]` vs `#[cfg(test)]`

The `#[test]` annotation on the `it_works` function indicates that this is a test function, so the test runner knows to treat this function as a test. The `#[cfg(test)]` annotation on the `tests` module tells Rust to compile and run the tests only when you run `cargo test`, not when you run `cargo build`.