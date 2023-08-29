pub struct SortAnalysis {
    pub compare: u32,
    pub swap: u32,
    pub arr_len: usize,
}

impl SortAnalysis {
    pub fn show_res(&self, sort_method: &str) {
        println!(
            "{}: {} memory cells are required, comparison {}, swap {}",
            sort_method, self.arr_len, self.compare, self.swap
        );
    }
}

pub fn merge_sort(vec: &mut [u32]) -> SortAnalysis {
    let mut analysis = SortAnalysis {
        compare: 0,
        swap: 0,
        arr_len: vec.len(),
    };

    if vec.len() < 2 {
        return analysis;
    }

    let mid = vec.len() / 2;
    merge_sort(&mut vec[..mid]);
    merge_sort(&mut vec[mid..]);

    let mut merged: Vec<u32> = Vec::new();
    merged.extend_from_slice(vec);

    let mut i = 0;
    let mut j = mid;
    let mut k = 0;

    analysis.compare += 1;
    while i < mid && j < vec.len() {
        if vec[i] < vec[j] {
            merged[k] = vec[i];
            i += 1;
        } else {
            merged[k] = vec[j];
            j += 1;
        }
        k += 1;

        analysis.compare += 1;
        analysis.swap += 1;
    }

    while i < mid {
        merged[k] = vec[i];
        i += 1;
        k += 1;

        analysis.compare += 1;
        analysis.swap += 1;
    }

    while j < vec.len() {
        merged[k] = vec[j];
        j += 1;
        k += 1;

        analysis.compare += 1;
        analysis.swap += 1;
    }

    vec.copy_from_slice(&merged);

    return analysis;
}

pub fn insertion_sort(vec: &mut [u32]) -> SortAnalysis {
    let mut res = SortAnalysis {
        compare: 0,
        swap: 0,
        arr_len: vec.len(),
    };

    for i in 1..vec.len() {
        let mut j = i;

        res.compare += 1;
        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;

            res.swap += 1;
            res.compare += 1;
        }
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge() {
        let mut vec = [1, 69, 2, 7, 5];
        merge_sort(&mut vec);

        assert_eq!(vec![1, 2, 5, 7, 69], vec);
    }
}
