# Multithreading and Concurrency [Overview]

Multithreading and Concurrency are important concepts in programming. They allow us to execute multiple tasks at the same time. This is important because it allows our programs to utilize the full potential of the CPU.

## Multithreading

Multithreading is a process of executing multiple threads simultaneously. Threads are lightweight sub-processes, which are separate from the main process. Multithreading is a specialized form of multitasking and a multitasking is the feature that allows your computer to run two or more programs concurrently.


In Rust, multithreading can be achieved in two ways:

- Using the `std::thread` module.

- Using the `std::sync` module.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
    println!("Hello from main thread!");

    // Output: 
    //Hello from a thread!
    //Hello from main thread!
}
```

In this example, we have created a thread using the `thread::spawn` function. The `thread::spawn` function takes a closure as an argument. The closure is the code that will be executed in the thread. The `thread::spawn` function returns a `JoinHandle` which is used to wait for the thread to finish execution.

The `handle.join().unwrap()` call will wait for the thread to finish execution. The `handle.join()` call will return a `Result` which we are unwrapping using the `unwrap` method. The `unwrap` method will panic if the thread has panicked. The `handle.join()` call will return `Ok(())` if the thread has finished execution successfully.


## Concurrency

Concurrency is the ability of a program to execute multiple tasks simultaneously. Concurrency is achieved by breaking a program into multiple independent tasks that can execute in parallel. Concurrency is achieved in Rust using the `std::sync` module.

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

    println!("Result is  {}", *counter.lock().unwrap());
}
// Output: Result is 10
```

In this example, we have created a `Mutex` using the `Mutex::new` function. The `Mutex` is a mutual exclusion primitive useful for protecting shared data. The `Mutex` provides interior mutability, which means that we can mutate the data even if there are immutable references to it. But we can't access the data without acquiring the lock first. The `Mutex` provides a `lock` method to acquire the lock. The `lock` method returns a `MutexGuard` which is a smart pointer that implements `Deref` to point to our data. The `MutexGuard` also implements `Drop` trait which automatically releases the lock when it goes out of scope.

The `Arc` is an atomic reference-counted pointer. The `Arc` is used to share ownership of the `Mutex` between multiple threads. The `Arc` provides a `clone` method to create a new reference to the `Mutex`. The `Arc` is atomic, which means that we can safely share it between multiple threads. The `Arc` is also immutable, which means that we can't mutate the data inside the `Arc` without acquiring the lock first.


## Conclusion

We learned in short that how to use multithreading and concurrency in Rust. We will learn more about multithreading and concurrency in the upcoming days.