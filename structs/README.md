# Structs

To define a struct, we enter the keyword `struct` and name the entire struct.

> Upper camelcase naming convention is used for naming structs in Rust.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To use the struct, we create an instance of it by specifying concrete values for each of the fields.

```rust
let user1 = User {
    email: String::from("abc@example.com"),
    username: String::from("abc"),
    active: true,
    sign_in_count: 1,
};
```

To get a specific value from a struct, we can use dot notation.

```rust
println!("Username: {}", user1.username);
```

We can also create a mutable instance of a struct.

```rust
let mut user2 = User {
    email: String::from("xyz@gmail.com"),
    username: String::from("xyz"),
    active: true,
    sign_in_count: 1,
};

user2.email = String::from("johndoe@gmail.com");
```

To print out the entire struct, we can use the `{:?}` format specifier. But we must also add the `#[derive(Debug)]` annotation to the struct definition.

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println("{:?}", user1);
    // Output: User { username: "abc", email: "abc@example", sign_in_count: 1, active: true }
}