# Shared Concurrency in Rust

Concurrency is a powerful tool for writing fast programs. Rust's type system and ownership model guarantee memory safety and thread safety, making concurrency much easier to use than in other languages. This book contains easy-to-understand explanations of Rust's concurrency tools, along with lots of sample code and practical exercises.


## Shared Concurrency

Shared concurrency is the idea that multiple threads can access the same data at the same time. This is in contrast to exclusive concurrency, where only one thread can access the data at a time. Shared concurrency is more difficult to implement, but it can be more efficient and flexible than exclusive concurrency.

Rust's type system and ownership model guarantee memory safety and thread safety, making shared concurrency much easier to use than in other languages. This book contains easy-to-understand explanations of Rust's concurrency tools, along with lots of sample code and practical exercises.


### Mutexes

A mutex is a mutual exclusion lock. It allows multiple threads to access the same data at the same time, but only one thread can access the data at a time. Mutexes are the most common way to implement shared concurrency in Rust. They are easy to use and efficient, but they can be difficult to understand.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("Value of m is {:?}", m);
    // Output: Value of m is Mutex { data: 5, poisoned: false, .. }
}
```

In this example, we create a mutex and store it in a variable named `m`. We then create a new scope with curly braces. This is called a _block_. Inside the block, we create a new variable named `num` and assign it the result of calling the `lock` method on the mutex. The `lock` method returns a `MutexGuard` smart pointer. The `MutexGuard` smart pointer implements the `Deref` and `Drop` traits. The `Deref` trait allows us to access the data inside the mutex. The `Drop` trait allows us to release the mutex when the `MutexGuard` goes out of scope.

Inside the block, we use the `*` operator to dereference the `MutexGuard` smart pointer and access the data inside the mutex. We then assign the value `6` to the data. When the `MutexGuard` goes out of scope, the mutex is released and the data is available for other threads to access.


### Arc

Arc is an atomic reference-counted pointer. It allows multiple threads to access the same data at the same time, but only one thread can modify the data at a time. Arc is the most common way to implement shared concurrency in Rust. It is used in place of `Rc` when shared concurrency is needed. Arc is easy to use and efficient, but it can be difficult to understand.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

In the above example, we create a mutex and store it in a variable named `counter`. We then create a new scope with curly braces. Inside the block, we create a new variable named `num` and assign it the result of calling the `lock` method on the mutex. The `lock` method returns a `MutexGuard` smart pointer. The `MutexGuard` smart pointer implements the `Deref` and `Drop` traits. The `Deref` trait allows us to access the data inside the mutex. The `Drop` trait allows us to release the mutex when the `MutexGuard` goes out of scope.

In this example, we create an `Arc` and store it in a variable named `counter`. We then create a new scope with curly braces. Inside the block, we create a new variable named `num` and assign it the result of calling the `lock` method on the mutex. The `lock` method returns a `MutexGuard` smart pointer. The `MutexGuard` smart pointer implements the `Deref` and `Drop` traits. The `Deref` trait allows us to access the data inside the mutex. The `Drop` trait allows us to release the mutex when the `MutexGuard` goes out of scope.