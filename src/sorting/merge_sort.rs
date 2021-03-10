use std::vec::Vec;

#[allow(dead_code)]

pub fn merge_sort(array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        return array;
    }
    let middle = (array.len() / 2) as usize;
    let left:&[i32] = &merge_sort((&array[..middle]).to_vec());
    let right:&[i32] = &merge_sort((&array[middle..]).to_vec());
    
    return merge(left, right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut result = Vec::new();
    
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
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

    return result
}