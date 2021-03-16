use std::vec::Vec;

#[allow(dead_code)]

pub fn count_inv(array: Vec<i32>) -> (Vec<i32>, usize) {
    if array.len() <= 1 {
        return (array, 0);
    }
    let middle = (array.len() / 2) as usize;
    let (left, left_count) = &count_inv((&array[..middle]).to_vec());
    let (right, right_count) = &count_inv((&array[middle..]).to_vec());
    let (merged, split_count) = merge_and_count_inv(left, right);
    
    return (merged, left_count + right_count + split_count)
}

fn merge_and_count_inv(left: &[i32], right: &[i32]) -> (Vec<i32>, usize) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut split_inv = 0;
    let mut result = Vec::new();
    
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
            split_inv += left.len() - left_index;
        }
    }
    
    while left_index < left.len() {
        result.push(left[left_index]);
        left_index += 1;
    }
    while right_index < right.len() {
        result.push(right[right_index]);
        right_index += 1;
    }

    return (result, split_inv)
}