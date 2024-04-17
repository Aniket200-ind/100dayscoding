# Macros

Macros in Rust are a way to define a pattern and then reuse it in multiple places. Macros are a way to write code that writes other code, which is known as metaprogramming. They are similar to functions, but instead of generating a value, they generate source code that gets compiled with the rest of the program.

## Declarative Macros with `macro_rules!` for General Metaprogramming

The most widely used form of macros in Rust is `macro_rules!`. This kind of macro is sometimes referred to as a declarative macro because it looks like a declaration: `macro_rules!`.

```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", v);
}
```

The `vec!` macro is similar to the `vec!` macro provided by the standard library, but it will only work for `u32` values. The `vec!` macro takes any number of arguments, creates a new `Vec`, puts those elements inside it, and returns the new `Vec`.

The `vec!` macro is an example of a `macro_rules!` macro that has a design similar to that of a match expression. The pattern `$( $x:expr ),*` matches zero or more expressions separated by commas. Inside the pattern, `$x` matches each expression, and the `*` says that the pattern matches zero or more of `$x`. The entire pattern is then followed by `=>`, which indicates the body of the macro. In this case, the body is relatively simple: it creates a new `Vec`, puts each expression in `$x` into the new `Vec`, and returns the new `Vec`.


## Procedural Macros for Generating Code from Attributes

Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do. Procedural macros come in three forms: custom `#[derive]` macros that specify code added with the derive attribute used on structs and enums, attribute-like macros that define custom attributes usable on any item, and function-like macros that look like function calls but operate on the tokens specified as their argument.

We have three types of procedural macros:-

### Custom `#[derive]` Macros

The most commonly used procedural macros are custom `#[derive]` macros. These macros specify code added with the `derive` attribute used on structs and enums. This is also the most common way you’ll see procedural macros used in the wild: by using a library that provides a custom `#[derive]`.

### Attribute-like Macros

Attribute-like macros define custom attributes that can be used on any item. For example, we could define a `route` attribute to annotate functions in our web server library to specify the URL to which requests should be routed. Using macros to specify this information means the function definition will look like this:

```rust
#[route(GET, "/")]

fn index() {
    // --snip--
}
```

### Function-like Macros

Function-like macros define macros that look like function calls. This macro `sql!` is an example of a function-like macro. Function-like macros are useful when you want to do something with the parameters passed to the macro. For example, the `println!` macro takes a format string and an arbitrary number of parameters. The format string parameter is passed to the macro exactly as specified by the caller, but the code that handles the `println!` macro parses the format string and determines how many additional parameters are specified and what types they have. This enables `println!` to be flexible enough to print many different kinds of values with one macro.

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```