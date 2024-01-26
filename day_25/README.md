# Closures in Rust

## What is a closure?

A closure is a function that can capture the enclosing environment. In other words, a closure can use variables from the scope in which it is defined. For example, a closure that captures the `x` variable:

```rust
let x = 4;

let equal_to_x = |z| z == x;
```

The closure `equal_to_x` can access the `x` variable because it is defined in the same scope as `x`. The closure is said to *close over* the `x` variable.

## Anonymous Functions

Closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined.

```rust
let x = 4;

let equal_to_x = |z| z == x;

let y = 4;

assert!(equal_to_x(y));
```

## Closure Type Inference and Annotation

Closures don’t require you to annotate the types of the parameters or the return value like `fn` functions do. Type annotations are required on functions because they’re part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. But closures aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.


## Traits: `Fn`, `FnMut`, `FnOnce`

Closures implement one of the traits: `Fn`, `FnMut`, or `FnOnce`. These traits are used to pass closures as arguments to functions.

```rust
fn main() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
}
```

The `FnOnce` trait is implemented by functions that take no arguments and return no values, so it matches the definition of the closure in the `main` function. The `apply` function takes a generic parameter `F` that is constrained so that `F` must implement the `FnOnce` trait. Any closure that satisfies the trait bound can be passed to the `apply` function as an argument.

## Capturing the Environment with Closures

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three `Fn` traits as follows:

- `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s *environment*. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The `Once` part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
- `FnMut` can change the environment because it mutably borrows values.
- `Fn` borrows values from the environment immutably.

When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment. All closures implement `FnOnce` because they can all be called at least once. Closures that don’t move the captured variables also implement `FnMut`, and closures that don’t need mutable access to the captured variables also implement `Fn`.

## Limitations of Closures

Closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. In these cases, the ability to name the closure and expose a public interface that is descriptive and documents exactly what the closure does is useful. Closures are anonymous, making that kind of documentation impossible. Additionally, in order to use a closure in more than one place in your code, you could extract the closure code into a function. But then you’d have to name that function, and functions can’t be defined inside other functions, so you’d have to move that function definition outside the function calling it.