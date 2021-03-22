use sorting::merge_sort;
use sorting::quick_sort;
use divide_and_conquer::count_inv;

mod sorting;
mod divide_and_conquer;

#[allow(dead_code)]

fn main() {
    let res = merge_sort(vec![1,8,3,13,4,6,12,5,7]);
    let (_, split_count) = count_inv(vec![1,3,5,2,4,6]);
    println!("{:?}", res);
    println!("{:?}", split_count);
    let mut a = vec![1,8,3,13,4,6,12,5,7];
    let len = a.len();
    quick_sort(&mut a, 0, len as usize);
    println!("{:?}", a);
}
