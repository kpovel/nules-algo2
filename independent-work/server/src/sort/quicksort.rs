use rand::Rng;

pub fn quick_sort<T: std::cmp::PartialOrd>(arr: &mut Vec<T>, left: isize, right: isize) {
    if left < right {
        let partition_idx = partition(arr, left, right);
        quick_sort(arr, left, partition_idx - 1);
        quick_sort(arr, partition_idx + 1, right);
    }
}

fn partition<T: std::cmp::PartialOrd>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot_index = rand::thread_rng().gen_range(left..=right) as usize;
    arr.swap(pivot_index, right as usize);

    let mut i: isize = left - 1;

    for j in left..right {
        if arr[j as usize] <= arr[right as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, right as usize);
    i + 1
}

