# Collection in Rust - Vector

Rust’s standard collection library provides efficient implementations of the most common general purpose programming data structures. By using the standard implementations, it should be possible for two libraries to communicate without significant data conversion.

## Vector

A vector allows you to store a variable number of values next to each other. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart. They are like dynamic arrays. They can grow or shrink in size, but have all the properties of arrays.

### Creating a Vector

```rust
let v: Vec<i32> = Vec::new();
v.push(5);
v.push(6);
v.push(7);

let v2 = vec![1, 2, 3];
```

### Reading Elements of Vectors

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

### Iterating over the Values in a Vector

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

There are many methods available for vectors.

[Documentation for the Vector]([text](https://doc.rust-lang.org/std/vec/struct.Vec.html))