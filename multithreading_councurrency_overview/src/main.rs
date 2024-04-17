use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let new_thread = thread::spawn(|| {
        println!("Hello from a new thread");
    });

    new_thread.join().unwrap();
    println!("Hello from the main thread");

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
