pub fn selection_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) {
    for i in 0..vec.len() - 1 {
        let mut min_n = i;
        for j in i + 1..vec.len() {
            if vec[j] < vec[min_n] {
                min_n = j;
            }
        }

        vec.swap(min_n, i);
    }
}

