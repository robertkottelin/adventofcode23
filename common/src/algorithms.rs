
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut base = 0;
    let mut size = slice.len();
    while size > 0 {
        let half = size / 2;
        let mid = base + half;
        if &slice[mid] <= target {
            if &slice[mid] == target {
                return Some(mid);
            }
            base = mid + 1;
            size -= half + 1;
        } else {
            size = half;
        }
    }
    None
}
