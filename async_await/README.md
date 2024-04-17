# Async/Await in Rust

Async/Await is a new feature in Rust 1.39.0. It allows you to write asynchronous code in a synchronous manner. This is a huge improvement over the previous asynchronous code, which was difficult to write and understand.


## What is asynchronous code?

Asynchronous code is code that can run in parallel with other code. This is useful for tasks that take a long time to complete, such as network requests or file I/O. In the past, asynchronous code was difficult to write and understand. Async/Await makes it much easier to write asynchronous code.

## How does it work?

```rust
#[tokio::main]
fn main() {
    tokio::spawn(async {
        hello_world().await;
    });
}

async fn hello_world() {
    println!("Hello, world!");
}
```

The `async` keyword tells the compiler that this function is asynchronous. The `await` keyword tells the compiler that this function will wait for the result of another asynchronous function before continuing.


## Future trait

The `Future` trait is the heart of async/await. It represents a value that will be available in the future. It can be used to represent any asynchronous operation, such as a network request or a file I/O operation. The `Future` trait is generic over two types: the type of the value that will be available in the future, and the type of the error that may occur while computing the value.


## Where it can be used?

Async/Await can be used in any Rust project. It is not limited to web development or any other specific domain. It can be used in any project that requires asynchronous code. It is especially useful for tasks that take a long time to complete, such as network requests or file I/O. It is also useful for tasks that require a lot of CPU time, such as image processing or video encoding. 