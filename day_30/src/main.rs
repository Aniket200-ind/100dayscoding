use std::fs::File;

#[tokio::main]
async fn main() {
    let f = my_function();
    println!("Hello from main function");
    f.await;
}

async fn my_function() {
    println!("I am an async function");
    let result1 = read_from_database().await;
    println!("First Result: {}", result1);
    let result2 = read_from_database().await;
    println!("Second Result: {}", result2);
}

async fn read_from_database() -> String {
    "Database result".to_owned()
}
