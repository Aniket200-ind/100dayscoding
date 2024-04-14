use std::fs::{remove_file, File};
use std::io::prelude::*;

fn write_file() -> std::io::Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn read_file() -> std::io::Result<()> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    Ok(())
}

fn update_file() -> std::io::Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello, Rustaceans!")?;
    Ok(())
}

fn delete_file() -> std::io::Result<()> {
    // * This will delete the file
    remove_file("hello.txt")?;
    Ok(())
}

fn main() {
    write_file().unwrap();
    read_file().unwrap();
    update_file().unwrap();
    read_file().unwrap();
}
