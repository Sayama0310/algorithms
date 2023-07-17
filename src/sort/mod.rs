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
