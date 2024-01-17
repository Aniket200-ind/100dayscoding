# HashMap

- HashMap is a data structure that stores items in the form of key-value pairs.

- It is similar to an array, except that indexing is done via arbitrary keys of any data type, not an integer index.

- It is also known as a hash table or associative array.

- It is a collection of key-value pairs.

- It is like a dictionary in python or an object in Javascript.


## HashMap Operations

- Insertion: Insert an item in the HashMap.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "20");
    println!("{:?}", map);
}
```

- Access: Access an item from the HashMap.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "20");
    println!("{:?}", map.get("name"));
}
```

- Deletion: Delete an item from the HashMap.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "20");
    map.remove("age");
    println!("{:?}", map);
}
```

- Update: Update an item in the HashMap.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "20");
    map.insert("age", "21");
    println!("{:?}", map);
}
```

- Iteration: Iterate over the HashMap.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "20");
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
}
```

- Length: Get the length of the HashMap.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "20");
    println!("{}", map.len());
}
```

## Real World use cases

- Storing user information in a web application.

> Later while using Rust as a backend language hashmaps can be useful!!

- Storing the number of occurrences of a word in a text document.