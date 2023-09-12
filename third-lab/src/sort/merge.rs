pub fn merge_sort(vec: &Vec<f64>) -> Vec<f64> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec());
        let right = merge_sort(&vec[size..].to_vec());
        let merged = merge(&left, &right);

        merged
    }
}

fn merge(left: &Vec<f64>, right: &Vec<f64>) -> Vec<f64> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<f64> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}

#[cfg(test)]
mod merge_sort {
    use crate::generate_arr;

    use super::merge_sort;

    #[test]
    fn worst() {
        let worst_arr = generate_arr(&crate::SortCase::WorstAsymptotic, 100_000);
        let mut worst_sorted = worst_arr.clone();
        worst_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let merge_sorted = merge_sort(&worst_arr);

        assert_eq!(merge_sorted, worst_sorted);
    }

    #[test]
    fn random() {
        let random_arr = generate_arr(&crate::SortCase::Random, 100_000);
        let mut random_sorted = random_arr.clone();
        random_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let merge_sorted = merge_sort(&random_arr);

        assert_eq!(merge_sorted, random_sorted);
    }

    #[test]
    fn best() {
        let best_arr = generate_arr(&crate::SortCase::BestAsymptotic, 100_000);
        let mut best_sorted = best_arr.clone();
        best_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let merge_sorted = merge_sort(&best_arr);

        assert_eq!(merge_sorted, best_sorted);
    }
}
