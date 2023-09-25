use super::SortStats;

pub fn merge_sort<T: Copy + PartialOrd>(vec: &Vec<T>) -> (Vec<T>, SortStats) {
    if vec.len() < 2 {
        (
            vec.to_vec(),
            SortStats {
                compare: 0,
                swap: 0,
                memory_usage: vec.len(),
            },
        )
    } else {
        let size = vec.len() / 2;
        let (left, left_stats) = merge_sort(&vec[0..size].to_vec());
        let (right, right_stats) = merge_sort(&vec[size..].to_vec());

        let (merged, merge_compares) = merge(&left, &right);

        let total_stats = SortStats {
            compare: left_stats.compare + right_stats.compare + merge_compares,
            swap: left_stats.swap + right_stats.swap + 1,
            memory_usage: vec.len(),
        };

        (merged, total_stats)
    }
}

fn merge<T: Copy + PartialOrd>(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, u32) {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<T> = Vec::new();
    let mut compares = 0;

    while i < left.len() && j < right.len() {
        compares += 1;
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }

    (merged, compares)
}
