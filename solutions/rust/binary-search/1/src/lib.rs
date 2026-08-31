pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut lo: usize = 0;
    let mut hi = array.len();

    while lo < hi {
        let pointer = lo + (hi - lo) / 2;
        let guess = array[pointer];

        if guess == key {
            return Some(pointer);
        } else if guess > key {
            hi = pointer;
        } else {
            lo = pointer + 1;
        }
    }

    None
}