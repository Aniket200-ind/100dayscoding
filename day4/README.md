# Compound data types

## Tuples

Tuples are used in Rust to store heterogenous data types together.

### Properties of tuple

- Fixed length

- Can store elements of different data types

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- Elements can be accessed using dot operator `.` and specifying the index of required element.

```rust
let tup: (i32, i32, i32) = (1, 2, 3);
let first = tup.0; // 1
let second = tup.1; //2
let third = tup.2; //3
```

- Tuples can be destructured to get individual elements

```rust
let tup = (1, 2, 3);
let (x, y, z) = tup;
println!("The value of x is {x}, the value of y is {y}, the value of z is {z}");
// The value of x is 1, the value of y is 2, the value of z is 3
```

> Note: The tuple without any values has a special name, **unit** `()` and it represents empty values or empty return type.

## Arrays

Arrays are used in Rust to store homogenous data types together.

Arrays are useful when you want your data allocated on the stack rather than the heap, in other words when we are sure that we will always have a fixed number of elements.

### Properties of array

- Fixed length

- Stores similar data types

- Elements can be accessed using square brackets `[]` and specifying the index of required element.

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0]; // 1
let second = a[1]; // 2
```

- Arrays can also be destructured to get individual elements

```rust
let a = [1, 2];
let [first, second] = a;
println!("The value of first is {first}, the value of second is {second}");
// The value of first is 1, the value of second is 2
```

## Differences between array and tuple

1. Arrays are homogenous, tuples are heterogenous.

2. Tuples are stored in heap memory, arrays are stored in stack memory.

3. Elements of array are stored in contiguous memory locations.

4. Tuples are initialized using paranthesis `()` and arrays are initialized using square brackets `[]`.

