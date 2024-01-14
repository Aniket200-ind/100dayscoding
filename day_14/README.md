# Generic Type and Traits

## Generic Type

- Generic type is a way to define a type that can be used with different concrete types.

- Generic type is declared with a type parameter.

- Type parameters are declared inside angle brackets.

- Type parameters are usually named with a single uppercase letter.


```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

- We can also define multiple type parameters.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

- We can also define functions that use generic types.

```rust
use std::ops::Add;

fn add<T: Add<Output = T> + Copy>(a: T, b: T) -> T {
    return a + b;
}

fn main() {
    let a = add(1, 2);
    let b = add(1.0, 2.0);
}
```

---

## Traits

- Traits are a way to define a set of methods that a type must implement.

- Traits are declared with the `trait` keyword.

- Traits are implemented with the `impl` keyword.

- Traits can have default implementations.

- Traits can be used as bounds for generic types.

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    dog.make_sound();
    cat.make_sound();
}
```