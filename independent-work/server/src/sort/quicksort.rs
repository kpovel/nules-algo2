use rand::Rng;

pub fn quick_sort<T: PartialOrd>(arr: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let partition_idx = partition(arr, left, right);
        quick_sort(arr, left, partition_idx - 1);
        quick_sort(arr, partition_idx + 1, right);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) -> usize {
    let pivot_index = rand::thread_rng().gen_range(left..=right);
    arr.swap(pivot_index, right);

    let mut i = left - 1;

    for j in left..right {
        if arr[j] <= arr[right] {
            i += 1;
            arr.swap(i, j);
        }
    }
    arr.swap(i + 1, right);
    i + 1
}
