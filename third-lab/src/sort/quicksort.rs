use rand::Rng;

pub fn quick_sort<T: std::cmp::PartialOrd>(arr: &mut Vec<T>, low: isize, high: isize) {
    if low < high {
        let partition_idx = partition(arr, low, high);
        quick_sort(arr, low, partition_idx - 1);
        quick_sort(arr, partition_idx + 1, high);
    }
}

fn partition<T: std::cmp::PartialOrd>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot_index = rand::thread_rng().gen_range(low..=high) as usize;
    arr.swap(pivot_index, high as usize);

    let pivot = high;
    let mut i: isize = low - 1;

    for j in low..high {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, pivot as usize);
    i + 1
}

#[cfg(test)]
mod quick_sort {
    use crate::{generate_arr, sort::quick_sort};

    #[test]
    fn worst() {
        let mut worst_arr = generate_arr(&crate::SortCase::WorstAsymptotic, 100_000);
        let mut worst_sorted = worst_arr.clone();
        worst_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let high = worst_arr.len() as isize - 1;
        quick_sort(&mut worst_arr, 0, high);

        assert_eq!(worst_arr, worst_sorted);
    }

    #[test]
    fn random() {
        let mut random_arr = generate_arr(&crate::SortCase::Random, 100_000);
        let mut random_sorted = random_arr.clone();
        random_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let high = random_arr.len() as isize - 1;
        quick_sort(&mut random_arr, 0, high);

        assert_eq!(random_arr, random_sorted);
    }

    #[test]
    fn best() {
        let mut best_arr = generate_arr(&crate::SortCase::BestAsymptotic, 100_000);
        let mut best_sorted = best_arr.clone();
        best_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let high = best_arr.len() as isize - 1;
        quick_sort(&mut best_arr, 0, high);

        assert_eq!(best_arr, best_sorted);
    }
}
