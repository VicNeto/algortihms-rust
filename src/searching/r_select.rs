use rand::prelude::*;

#[allow(dead_code)]

pub fn r_select(array: &mut Vec<usize>, i_pos: usize) -> usize{
    if array.len() == 1 {
        return array[0];
    }
    let pivot = choose_pivot(array.len());
    array.swap(0, pivot);
    let position = partition(array);

    if position == i_pos {
        return array[position]
    } else if position > i_pos {
        return r_select(&mut (array[..position]).to_vec(), i_pos);
    }
    return r_select(& mut (array[position+1..]).to_vec(), i_pos-position-1);
}

fn partition(array: &mut [usize]) -> usize {
    let mut i = 1;
    for j in 1..(array.len()) {
        if array[j] < array[0] {
            array.swap(j, i);
            i += 1;
        }
    }
    array.swap(0, i - 1);
    return i - 1
}

fn choose_pivot(length: usize) -> usize {
    let mut rng = thread_rng();
    return rng.gen_range(0..length);
}
