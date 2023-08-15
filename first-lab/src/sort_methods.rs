pub fn bubble_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

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

pub fn insertion_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) {
    for i in 1..vec.len() {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}

