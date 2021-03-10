use sorting::merge_sort;

mod sorting;

#[allow(dead_code)]

fn main() {
    // let res = merge(&vec![3,7,9], &vec![1,8,10]);
    let res = merge_sort(vec![1,8,3,13,4,6,12,5,7]);
    println!("{:?}", res);
}
