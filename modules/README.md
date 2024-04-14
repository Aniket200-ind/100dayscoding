# Modules in Rust

## What is a module?

- A module is a collection of items: functions, structs, traits, impl blocks, and even other modules. 
- Modules help us organize code within a crate into groups for readability and easy reuse. 
- Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

## Module declaration

- Modules are declared using the `mod` keyword.

```rust
mod sound {
    fn guitar() {
        // Function body code goes here
        println("Guitar sound");
    }
}
```

- The `mod` keyword declares a new module, and the body of the module is contained in the curly brackets.

## Module paths

- Module paths are used to locate modules and the items in them.

```rust
mod sound {
    fn guitar() {
        // Function body code goes here
        println("Guitar sound");
    }
}

mod performance_group {
    // Absolute path
    crate::sound::guitar();

    // Relative path
    sound::guitar();
}
```

- Module paths can be either absolute or relative.

- To refer to an item in the current module, we can use a relative path.


## Declaring Module in different files

- We can declare modules in different files and use them in the main file.

- We can use the `pub` keyword to make the module public.

- We can use the `use` keyword to bring the module into scope.

- This makes code more readable and maintainable.

```rust
// main.rs
mod sound;

fn main() {
    sound::guitar();
}
```

---

```rust
// sound.rs
pub mod sound {
    pub fn guitar() {
        // Function body code goes here
        println("Guitar sound");
    }
}
```