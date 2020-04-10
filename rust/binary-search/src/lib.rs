use std::cmp::Ordering;

pub fn find<A: AsRef<[T]>, T: Ord>(array: A, key: T) -> Option<usize> {
    let slice = array.as_ref();
    if slice.is_empty() {
        return None;
    }
    let mid = slice.len() / 2;
    match key.cmp(&slice[mid]) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&slice[0..mid], key),
        Ordering::Greater => find(&slice[mid + 1..], key).map(|i| i + mid + 1),
    }
}
