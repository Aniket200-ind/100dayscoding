# Ownership, Referencing and Borrowing

> **Note:** I guess this is a more theorotical concept to learn so examples won't help much.

[Link to ownership documentaton](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

[Link to References and Borrowing documentation](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

[Link to my Blog](https://dev.to/aniket_botre/day-9-navigating-ownership-references-and-borrowing-in-rust-21ec)

---

## Ownership

### What is Ownership?

Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Therefore, it's important to understand how ownership works in Rust.

> In many programming languages, you don't have to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap has more of an effect on how the language behaves and why you have to make certain decisions.

Rust stores primitive datatypes directly into the memory but complex datatypes are generally stored in either Stack (or) Heap.

### Rules of ownership

- Each value in Rust has a variable called its owner.

- There can only be one owner at a time.

- When the owner goes out of scope, the value will be dropped.

Here is a simlple example to demonstrate above rules

```rust
fn main() {
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no longer valid
```

---

## References and Borrowing

- References allow you to refer to some value without taking ownership of it.

- We call having references as function parameters borrowing.

- As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

### Mutable References

- We can have only one mutable reference to a particular piece of data in a particular scope.

- We also cannot have a mutable reference while we have an immutable one.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
