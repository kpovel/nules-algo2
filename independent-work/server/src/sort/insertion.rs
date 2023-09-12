pub fn insertion_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) {
    for i in 1..vec.len() {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}
