use rand::Rng;

use super::InitSort;

mod bubble;
mod insertion;
mod merge;
mod quicksort;
mod selection;

pub use {
    bubble::bubble_sort, insertion::insertion_sort, merge::merge_sort, quicksort::quick_sort,
    selection::selection_sort,
};

pub struct SortStats {
    pub compare: u32,
    pub swap: u32,
    pub memory_usage: usize,
}

pub fn generate_vec(vec_type: &InitSort, len: usize) -> Vec<f64> {
    match vec_type {
        InitSort::Sorted => (0..len).map(|x| x as f64).collect(),
        InitSort::Random => {
            let mut vec: Vec<f64> = vec![];

            for _ in 0..len {
                vec.push(rand::thread_rng().gen_range(0.0..len as f64));
            }
            println!("{:?}", vec);

            return vec;
        },
        InitSort::ReverceSorted => (0..len).map(|x| x as f64).rev().collect(),
    }
}
