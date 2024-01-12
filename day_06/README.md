# Loops

Rust has three types of loops:

## 1. Loop Statement

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

`break` keyword tells Rust to stop executing the loop.

```rust
let counter = 1;
loop {
    println!("{counter}");
    if counter <= 5 {
        break;
    }
    counter += 1;
}
//  Output:
1
2
3
4
5
```

In the above example, the `counter` variable is initialized to 1. The loop will run forever until the `counter` variable is less than or equal to 5. The `break` keyword will stop the loop when the `counter` variable is equal to 6.

## 2. While Loop

The `while` loop executes a block of code as long as a condition is true.

```rust
let mut counter = 1;
while counter <= 5 {
    println!("{counter}");
    counter += 1;
}
//  Output:
1
2
3
4
5
```

In the above example, the `counter` variable is initialized to 1. The loop will run as long as the `counter` variable is less than or equal to 5. The `counter` variable is incremented by 1 in each iteration. The loop will run as long as the condition is true i.e. here counter is less than or equal to 5.

## 3. For Loop

The `for` loop is used to iterate over a collection of items. The `for` loop is used to loop over a range of numbers, an array, a vector, a string, or any collection that implements the `Iterator` trait.

- Example 1:

```rust
let numbers = [1, 2, 3, 4, 5];
for element in numbers {
    println!("{element}");
}
//  Output:
1
2
3
4
5
```

In the above example, the `numbers` variable is an array of numbers. The `for` loop will iterate over the `numbers` array and print each number.

---

- Example 2:

```rust
for i in range 1..=10 {
    println!("{i}");
}
//  Output:
1
2
3
4
5
6
7
8
9
10
```

In the above example, the `for` loop will iterate over the range of numbers from 1 to 10 **inclusive** and print each number.

> There are various functions from `Iterator` trait available in for loop. I still didn't completely understood how they work. But you can check it out here: [For and Iterator](https://doc.rust-lang.org/rust-by-example/flow_control/for.html)
