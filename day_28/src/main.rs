use std::{sync::mpsc, thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the first spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..=5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }

    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("{} from second spawn thread", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 11..=15 {
        println!("{} from main thread", i);
        thread::sleep(Duration::from_secs(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3, 4, 5];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
