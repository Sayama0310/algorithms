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

pub fn quick_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    let pivot = partition(array);
    quick_sort(&mut array[0..pivot]);
    quick_sort(&mut array[pivot + 1..len]);
}

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let len = array.len();
    let pivot = len - 1;
    let mut i = 0;
    for j in 0..len - 1 {
        if array[j] < array[pivot] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, pivot);
    i
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

    #[test]
    fn test_quick_sort() {
        let mut array = [1, 5, 2, 3, 4];
        quick_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }
}
