# File handling in Rust

File handling in Rust is done through the `std::fs` module. This module contains functions for opening, reading, writing and closing files. The functions are used by importing the module into the scope of the program.

`?` operator is used to handle errors. If an error occurs, the function returns the error to the caller. The caller can then handle the error in a way that is appropriate for the program.

```rust
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
```

In this example,

- `write_file()` creates a file named `hello.txt` and writes `Hello, world!` to it.

- `read_file()` opens the file `hello.txt` and reads its contents into a string.

- `update_file()` opens the file `hello.txt` and writes `Hello, Rustaceans!` to it.

- `delete_file()` deletes the file `hello.txt`.

## Reading a file line by line

```rust
use std::fs::File;

fn main() {
    let file = File::open("hello.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
```
