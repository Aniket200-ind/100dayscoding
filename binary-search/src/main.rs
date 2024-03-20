use std::cmp::Ordering;

fn main() {
    let array = [1, 2, 3, 4, 5, 16, 17, 18, 19, 20];
    let key = 19;
    let result = find(&array, key);
    match result {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found"),
    }
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mid = array.len() / 2;
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&array[..mid], key),
        Ordering::Greater => find(&array[mid + 1..], key).map(|i| i + mid + 1),
    }
}
