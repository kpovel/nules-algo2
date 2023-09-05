mod sort;
use std::array;

use rand::Rng;
use sort::{insertion_sort, merge_sort, SortAnalysis};

const ARR_LEN: usize = 30 + 9 * 2 - (9 / 2);

fn main() {
    let mut insertion_best_arr: [u32; ARR_LEN] = array::from_fn(|i| i as u32 + 1);
    let insertion_best = insertion_sort(&mut insertion_best_arr);
    insertion_best.show_res("Insetion best");

    let mut insertion_rand_stats: Vec<SortAnalysis> = vec![];

    for _ in 0..100 {
        let mut insertion_rand_arr = generate_rand_arr();
        let insertion_random = insertion_sort(&mut insertion_rand_arr);
        insertion_rand_stats.push(insertion_random);
    }

    show_rand_result(insertion_rand_stats, "Insertion random");

    let mut insertion_worst_arr: [u32; ARR_LEN] = array::from_fn(|i| i as u32 + 1);
    insertion_worst_arr.reverse();
    let insertion_worst = insertion_sort(&mut insertion_worst_arr);
    insertion_worst.show_res("Insetion worst");

    let mut merge_best_arr: [u32; ARR_LEN] = array::from_fn(|i| i as u32 + 1);
    let merge_best = merge_sort(&mut merge_best_arr);
    merge_best.show_res("Merge best");

    let mut merge_rand_stats: Vec<SortAnalysis> = vec![];

    for _ in 0..100 {
        let mut merge_rand_arr = generate_rand_arr();
        let merge_random = merge_sort(&mut merge_rand_arr);
        merge_rand_stats.push(merge_random);
    }

    show_rand_result(merge_rand_stats, "Merge random");

    let mut merge_worst_arr: [u32; ARR_LEN] = array::from_fn(|i| i as u32 + 1);
    merge_worst_arr.reverse();
    let merge_worst = merge_sort(&mut merge_worst_arr);
    merge_worst.show_res("Merge worst");
}

fn generate_rand_arr() -> [u32; ARR_LEN] {
    let mut arr: [u32; ARR_LEN] = [0; ARR_LEN];

    for i in 0..ARR_LEN {
        arr[i] = rand::thread_rng().gen_range(0..100);
    }

    return arr;
}

fn show_rand_result(stats: Vec<SortAnalysis>, sort_name: &str) {
    let mut avg_comparisons: f32 = 0.0;
    let mut avg_swaps: f32 = 0.0;
    let memory_cells = stats[0].arr_len;

    for stat in stats.iter() {
        avg_comparisons += stat.compare() as f32;
        avg_swaps += stat.swap() as f32;
    }

    avg_comparisons /= stats.len() as f32;
    avg_swaps /= stats.len() as f32;

    let stats = SortAnalysis {
        compare: avg_comparisons as u32,
        swap: avg_swaps as u32,
        arr_len: memory_cells,
    };

    stats.show_res(sort_name);
}
