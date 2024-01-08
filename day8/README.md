# Strings

> String data type makes a very important part of any programming language. Rust handles strings a bit differently from other languages.


In Rust, strings come in two main forms: string literals and the dynamic `String` type.

> **Note:** Understanding the differences between String and str in Rust is crucial for writing efficient and memory-safe code.

---

## String literals

> **Note:** Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.

String Literal or `&str` are called ‘string slices’, which always point to a legitimate UTF-8 sequence. It is used when we know the value of a string at compile time. They are a set of characters and static by default.

```rust
let static_str: &str = "Hello, world!";
println(s);
// Output: Hello, world!
```

The `static_str` is a string literal with a fixed size, and it's immutable. String literals are convenient but lack the flexibility of dynamic strings.

---

## The `String` object

> The String type in Rust is essentially a dynamic array of bytes (Vec<u8>)

The `String` Object is provided by the Standard Library in Rust. It is not a part of the core language and String is heap-allocated, growable, mutable, and not null-terminated.

```rust
let mut dynamic_str = String::from("Hello, ");
dynamic_str.push_str("Rust!");
println!("{}", dynamic_str);
// Output: Hello, Rust!
```

There are several methods to create a String in Rust:

- Using `String::new()` to create an empty string.

- Using string literals and converting them with `.to_string()` or `String::from()`.

String also provides various methods for string manipulation, such as:

- Appending strings with `.push_str()`.

```rust
let mut dynamic_str = String::new();
dynamic_str.push_str("Hello World!");
println!("{}", dynamic_str);
// Output: Hello World!
```

- Concatenating strings with `format!()`.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
println!("{}", s);
// Output: tic-tac-toe
```

- Replacing substrings with `.replace()`.

```rust
let s = String::from("Hello, world!");
let s = s.replace("world", "Rustaceans");
println!("{}", s);
// Output: Hello, Rustaceans!
```

- Trimming whitespace using `.trim()`

```rust
let s = String::from("   Hello, world!   ");
let s = s.trim();
println!("{}", s);
// Output: Hello, world!
```

## Important points to note

- Rust strings are UTF-8 encoded by default.

- String literals in Rust are of the `&str` type and are stored directly in the executable's memory, making them efficient and fast to access.

- Creating a new String triggers an allocation, which can impact runtime performance . Therefore, it is recommended to use string slices (`&str`) when possible to avoid additional allocations.

- In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.


