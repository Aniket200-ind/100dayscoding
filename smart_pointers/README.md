# Smart pointers in Rust

## What is a smart pointer?

A smart pointer is a data structure that wraps a pointer and adds additional information and functionality to it. The additional information is usually metadata such as the size of the allocation, the number of references to the allocation, the type of the allocation, etc. The additional functionality is usually related to the lifetime of the allocation, such as allocating and deallocating the allocation, or automatically deallocating the allocation when the smart pointer goes out of scope.

## Why use a smart pointer?

Smart pointers are useful because they allow you to have more control over the lifetime of the allocation. For example, you can allocate an object on the heap and then pass it to a function that takes ownership of the object. The function can then return the object to you, and you can use it again. This is useful because it allows you to reuse the object without having to allocate it again.

## Some examples of smart pointers in Rust

### Box

The `Box` type is a smart pointer that allows you to allocate an object on the heap and then pass it to a function that takes ownership of the object. The function can then return the object to you, and you can use it again. This is useful because it allows you to reuse the object without having to allocate it again.

```rust
fn main() {
    let x = Box::new(5);
    println!("x = {}", x);
//  Output: x = 5
}
```

In this example, we allocate an integer on the heap and then pass it to a function that takes ownership of the integer. The function can then return the integer to us, and we can use it again. This is useful because it allows us to reuse the integer without having to allocate it again.


### Rc

The `Rc` type is a smart pointer that allows you to have multiple owners of an object. This is useful because it allows you to share an object between multiple threads without having to copy the object.

```rust
use std::rc::Rc;

fn main() {
    let x = Rc::new(5);
    let y = x.clone();
    let count = Rc::strong_count(&x);
    println!("x = {}, y = {}", x, y);
    println!("count = {}", count);
//  Output: x = 5, y = 5
//  Output: count = 2
}
```

In this example, we create an `Rc` pointer to an integer and then clone it. The `Rc` pointer has a reference count of 2, so when we print the reference count, we get 2. This is useful because it allows us to share an integer between multiple threads without having to copy the integer.


### RefCell

The `RefCell` type is a smart pointer that allows you to have multiple mutable references to an object. This is useful because it allows you to share an object between multiple threads without having to copy the object.

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let y = x.borrow_mut();
    let z = x.borrow_mut();
    println!("x = {}, y = {}, z = {}", x, y, z); 
//  Output: x = 5, y = 5, z = 5
}
```

In this example, we create a `RefCell` pointer to an integer and then borrow it mutably twice. The `RefCell` pointer has a reference count of 2, so when we print the reference count, we get 2. This is useful because it allows us to share an integer between multiple threads without having to copy the integer.
