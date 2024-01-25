# Merge Sort in Rust

Today for day24 of #100daysofcode in Rust, I decided to implement merge sort. I have implemented merge sort in C++ and JavaScript before, but I wanted to try it in Rust. I also wanted to try to implement it in a more functional style, so I used recursion instead of loops. I also used slices instead of vectors, which I think is more idiomatic Rust.


## What is Merge Sort?

Merge sort is a sorting algorithm that uses the divide and conquer strategy. It divides the input array into two halves, calls itself for the two halves, and then merges the two sorted halves. The merge function is used for merging two halves. The merge(arr, l, m, r) is a key process that assumes that arr[l..m] and arr[m+1..r] are sorted and merges the two sorted sub-arrays into one. See [Wikipedia](https://en.wikipedia.org/wiki/Merge_sort) for more information.

## Implementation

```rust
fn merge_sort(num_arr: &[i32]) -> Vec<i32>{
    let length = num_arr.len();

    if length < 2 {
        return Vec::from(num_arr);
    }

    let mid = length / 2;
    let left = &num_arr[0..mid];
    let right = &num_arr[mid..length];

    let mut sorted_left = merge_sort(&left);
    let mut sorted_right = merge_sort(&right);

    let sorted_arr = merge(&mut sorted_left, &mut sorted_right);

    sorted_arr
}
```

The `merge_sort` function takes a slice of integers and returns a vector of integers. If the length of the slice is less than 2, it returns the slice as a vector. Otherwise, it splits the slice into two halves, calls itself for the two halves, and then merges the two sorted halves. The `merge` function is used for merging two halves.

---

```rust
fn merge(sorted_array1: &mut Vec<i32>, sorted_array2: &mut Vec<i32>) -> Vec<i32>{
    let mut result: Vec<i32> = Vec::new();

    while !sorted_array1.is_empty() && !sorted_array2.is_empty() {
        if sorted_array1[0] <= sorted_array2[0] {
            result.push(sorted_array1.remove(0));
        } else{
            result.push(sorted_array2.remove(0));
        }
    }

    result.extend_from_slice(sorted_array1);
    result.extend_from_slice(sorted_array2);

    result
}
```

The `merge` function takes two mutable vectors of integers and returns a vector of integers. It creates an empty vector to store the result. It then compares the first elements of the two vectors and pushes the smaller one to the result vector. It repeats this process until one of the vectors is empty. It then appends the remaining elements of the non-empty vector to the result vector. It then returns the result vector.

---

## Testing

```rust
fn main() {
    let arr = [10, 5, 3, 8, 2, 6, 4, 7, 9, 1];
    println!("Array before sorting {:?}", arr);
    println!("Array after sorting {:?}", merge_sort(&arr));
}
```

I created a main function to test the merge sort function. I created an array of integers and printed it to the console. I then called the merge sort function and printed the result to the console.

### Output

```bash
Array before sorting [10, 5, 3, 8, 2, 6, 4, 7, 9, 1]
Array after sorting [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```