# Multithreading in Rust

## Spawning a thread

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
```

In the above example, the spawned thread is not guaranteed to finish before the main thread. If we want to wait for the spawned thread to finish, we need to call `join` on the handle.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join().unwrap();
}
```

As a result, the main thread will wait for the spawned thread to finish before exiting. The `join` method also returns a `Result` type, so we need to call `unwrap` to get the result.


## Ownership of the environment

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

Rust provides the `move` keyword to force the closure to take ownership of the values it uses in the environment. In the above example, the spawned thread takes ownership of `v` and the main thread cannot use `v` anymore.
If we try to use `v` in the main thread, we will get a compile error.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    println!("Here's a vector: {:?}", v);
}
```

Output:

```bash
error[E0382]: borrow of moved value: `v`
 --> src/main.rs:11:38
  |
5 |     let handle = thread::spawn(move || {
    |                              ------- value moved (into closure) here
...
11 |     println!("Here's a vector: {:?}", v);
    |                                      ^ value borrowed here after move
    |
    = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
```


## Using message passing to transfer data between threads

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

The `mpsc` stands for multiple producer, single consumer. The `tx` is the sending end and the `rx` is the receiving end. The `send` method takes ownership of the value and moves it along the channel. The `recv` method returns a `Result` type, so we need to call `unwrap` to get the value.

In a program there can be multiple producers and a single consumer. The `clone` method can be used to create multiple sending ends.

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
    });

    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("from the second thread");
        tx2.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```


Multithreading makes code logic complex to understand and debug. So we need to be careful when using multithreading.