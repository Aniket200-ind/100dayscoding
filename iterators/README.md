# Iterators in Rust

Iterators in Rust are a powerful feature that allows you to perform operations on a sequence of elements. They are a fundamental part of Rust's standard library and are used in many places, such as for loops, map, filter, and collect methods.

## What is an Iterator?

An iterator is a trait that represents a sequence of elements. It provides a way to iterate over a collection of items without exposing the underlying data structure. This allows you to perform operations on the elements without having to know the details of the data structure.

The Iterator trait has a single method, next, which returns an Option containing the next element in the sequence. When the iterator is exhausted, it returns None.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The Item associated type represents the type of the elements in the sequence. The next method returns an Option containing the next element in the sequence, or None if the iterator is exhausted.

## Creating an Iterator

You can create an iterator from a collection using the iter method. This method returns an iterator over the elements of the collection.

```rust
let v = vec![1, 2, 3];
let mut iter = v.iter();
```

In the above example, we create an iterator over the elements of a vector using the iter method. The iter method returns an iterator over the elements of the vector.

## Consuming an Iterator

You can consume an iterator by calling the next method in a loop. This will iterate over the elements of the iterator until it is exhausted.

```rust
let v = vec![1, 2, 3];
let mut iter = v.iter();
loop {
    match iter.next() {
        Some(x) => println!("{}", x),
        None => break,
    }
}
```

In the above example, we consume the iterator by calling the next method in a loop. This will iterate over the elements of the iterator until it is exhausted.


## Using Iterator Methods

Rust provides a number of methods on the Iterator trait that allow you to perform operations on the elements of the sequence. These methods include map, filter, and collect, among others.

```rust
let v = vec![1, 2, 3];

let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

In the above example, we use the map method to double each element in the vector, and then collect the results into a new vector.
