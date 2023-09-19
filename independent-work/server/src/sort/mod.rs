use rand::Rng;

use super::VecType;

mod bubble;
mod insertion;
mod merge;
mod quicksort;
mod selection;

pub use {
    bubble::bubble_sort, insertion::insertion_sort, merge::merge_sort, quicksort::quick_sort,
    selection::selection_sort,
};

pub fn generate_vec(vec_type: &VecType, len: usize) -> Vec<f64> {
    match vec_type {
        VecType::Sorted => (0..len).map(|x| x as f64).collect(),
        VecType::Random => {
            let mut vec: Vec<f64> = vec![];

            for _ in 0..len {
                vec.push(rand::thread_rng().gen_range(0.0..len as f64));
            }
            println!("{:?}", vec);

            return vec;
        },
        VecType::ReverceSorted => (0..len).map(|x| x as f64).rev().collect(),
    }
}
