fn merge_sort(num_arr: &[i32]) -> Vec<i32> {
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

fn merge(sorted_array1: &mut Vec<i32>, sorted_array2: &mut Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    while !sorted_array1.is_empty() && !sorted_array2.is_empty() {
        if sorted_array1[0] <= sorted_array2[0] {
            result.push(sorted_array1.remove(0));
        } else {
            result.push(sorted_array2.remove(0));
        }
    }

    result.extend_from_slice(sorted_array1);
    result.extend_from_slice(sorted_array2);

    result
}

fn main() {
    let arr = [10, 5, 3, 8, 2, 6, 4, 7, 9, 1];
    println!("Array before sorting {:?}", arr);
    println!("Array after sorting {:?}", merge_sort(&arr));
}
