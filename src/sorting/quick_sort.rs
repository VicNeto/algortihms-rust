
#[allow(dead_code)]

pub fn quick_sort(array: &mut [u64], left: usize, right: usize) {
    if left >= right {
        return
    }
    let pivot = right - 1;
    array.swap(left, pivot);
    let new_pivot = partition(array, left, right);
    quick_sort(array, left, new_pivot);
    quick_sort(array, new_pivot + 1, right);
}

pub fn partition(array: &mut [u64], left: usize, right: usize) -> usize {
    let mut i = left + 1;
    for j in (left + 1)..right {
        if array[j] < array[left] {
            array.swap(j, i);
            i += 1;
        }
    }
    array.swap(left, i - 1);
    return i - 1
}
