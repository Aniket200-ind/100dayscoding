# Lifetimes in Rust

## Dangling Pointers

Dangling pointers are pointers that reference a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory. 

This is a common error in C and C++ programming languages because they have no garbage collector.

However, Rust compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does with help of Lifetimes. 

## Liftimes

The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

Lifetimes are a way of annotating the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

They solve the problem of dangling references by adding a constraint to the references.

Lifetime annotations don't change how long any of the references live.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

The above code will not compile because the reference `r` does not live long enough.The compiler throws an error because the reference `r` is not valid because it does not live long enough.


### Liftimes in function signature

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The above code will compile because the compiler knows that the reference returned by the `longest` function will live at least as long as the smaller of the lifetimes of `x` and `y`.

### Lifetime Annotations in Struct Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

The above code will compile because the struct `ImportantExcerpt` contains a reference, so we must annotate it with a lifetime as well.


## Conclusion

Lifetimes ensure that the data referenced by a certain value will not go out of scope before the value does. It is an important concept in Rust that allows Rust to make memory safety guarantees without needing a garbage collector.

