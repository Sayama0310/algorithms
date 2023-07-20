pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

/// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut array = [1, 5, 2, 3, 4];
        bubble_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }
}
