# Enums

Enums, short for enumerations, are a feature of Rust that allow you to define a type by enumerating its possible variants. Here is an example:

```rust
enum Genre {
    Pop,
    Rock,
    HipHop,
    Jazz,
}
```

---

## Pattern matching

We can use pattern matching to match against the different variants of an enum. For example:

```rust
let my_genre = Genre.Pop;

match my_genre {
    Genre::Pop => println!("Pop"),
    Genre::Rock => println!("Rock"),
    Genre::HipHop => println!("HipHop"),
    Genre::Jazz => println!("Jazz"),
}
```

---

We can then use this enum in a function like this:

```rust
fn genre_match(genre: Genre) {
    match genre {
        Genre::Pop => println!("Pop"),
        Genre::Rock => println!("Rock"),
        Genre::HipHop => println!("HipHop"),
        Genre::Jazz => println!("Jazz"),
    }
}
```

---

We can also use enums in structs:

```rust
struct Song {
    name: String,
    genre: Genre,
}
```

---

## Variant types

Enums can also have data associated with each variant. For example:

```rust
enum Genre {
    Pop,
    Rock,
    HipHop,
    Jazz,
    Other(String),
}
```

---

We can also use methods with enums:

```rust
impl Genre {
    fn print(&self) {
        match self {
            Genre::Pop => println!("Pop"),
            Genre::Rock => println!("Rock"),
            Genre::HipHop => println!("HipHop"),
            Genre::Jazz => println!("Jazz"),
            Genre::Other(s) => println!("{}", s),
        }
    }
}
```