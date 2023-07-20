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

pub fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut array[0..mid]);
    merge_sort(&mut array[mid..len]);
    merge(array, mid);
}

fn merge<T: Ord + Copy>(array: &mut [T], mid: usize) {
    let len = array.len();
    let mut temp = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;
    while i < mid && j < len {
        if array[i] < array[j] {
            temp.push(array[i]);
            i += 1;
        } else {
            temp.push(array[j]);
            j += 1;
        }
    }
    if i < mid {
        temp.extend_from_slice(&array[i..mid]);
    }
    if j < len {
        temp.extend_from_slice(&array[j..len]);
    }
    array.copy_from_slice(&temp);
}

pub fn heap_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    for i in (0..len / 2).rev() {
        heapify(array, len, i);
    }
    for i in (1..len).rev() {
        array.swap(0, i);
        heapify(array, i, 0);
    }
}

fn heapify<T: Ord>(array: &mut [T], len: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    if left < len && array[left] > array[largest] {
        largest = left;
    }
    if right < len && array[right] > array[largest] {
        largest = right;
    }
    if largest != i {
        array.swap(i, largest);
        heapify(array, len, largest);
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

    #[test]
    fn test_quick_sort() {
        let mut array = [1, 5, 2, 3, 4];
        quick_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        let mut array = [1, 5, 2, 3, 4];
        merge_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heap_sort() {
        let mut array = [1, 5, 2, 3, 4];
        heap_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }
}
