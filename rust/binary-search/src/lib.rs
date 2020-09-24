use core::cmp::Ordering::{Equal, Greater, Less};

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mid_index = array.len() / 2;
    let mid_val = array.get(mid_index)?;
    match key.cmp(mid_val) {
        Less => find(&array[..mid_index], key),
        Equal => Some(mid_index),
        Greater => find(&array[mid_index + 1..], key).map(|res| res + mid_index + 1),
    }
}
