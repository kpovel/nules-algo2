use super::SortStats;

pub fn insertion_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) -> SortStats {
    let mut stats = SortStats {
        compare: 0,
        swap: 0,
        memory_usage: vec.len(),
    };

    for i in 1..vec.len() {
        let mut j = i;

        while j > 0 {
            stats.compare += 1;

            if vec[j] < vec[j - 1] {
                vec.swap(j, j - 1);
                stats.swap += 1;
                j -= 1;
            } else {
                break;
            }
        }
    }

    stats
}
