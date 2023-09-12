use rand::Rng;
use sort::{insertion_sort, merge_sort, quick_sort};
use std::time::{Duration, Instant};

mod sort;

const ARR_LEN: [u32; 5] = [50_000, 100_000, 150_000, 200_000, 250_000];

enum SortCase {
    WorstAsymptotic,
    BestAsymptotic,
    Random,
}

fn main() {
    test_sort(SortCase::WorstAsymptotic);
    test_sort(SortCase::Random);
    test_sort(SortCase::BestAsymptotic);
}

fn test_sort(sort_case: SortCase) {
    match sort_case {
        SortCase::WorstAsymptotic => println!("Worst attempt: "),
        SortCase::BestAsymptotic => println!("Best attempt: "),
        SortCase::Random => println!("Random attempt: "),
    };

    for n in ARR_LEN {
        let mut quicksort_arr = generate_arr(&sort_case, n);

        let start = Instant::now();
        quick_sort(&mut quicksort_arr, 0, n as isize - 1);
        let end = Instant::now();

        show_res("Quicksort", quicksort_arr.len(), end - start);
    }

    println!("");
    for n in ARR_LEN {
        let mut merge_arr: Vec<f64> = generate_arr(&sort_case, n);

        let start = Instant::now();
        merge_sort(&mut merge_arr);
        let end = Instant::now();

        show_res("Merge sort", merge_arr.len(), end - start);
    }

    println!("");
    for n in ARR_LEN {
        let mut merge_arr: Vec<f64> = generate_arr(&sort_case, n);

        let start = Instant::now();
        insertion_sort(&mut merge_arr);
        let end = Instant::now();

        show_res("Insertion sort", merge_arr.len(), end - start);
    }
    println!("");
}

fn generate_arr(sort_case: &SortCase, vec_len: u32) -> Vec<f64> {
    match sort_case {
        SortCase::WorstAsymptotic => (0..vec_len).map(|x| x as f64).rev().collect(),
        SortCase::BestAsymptotic => (0..vec_len).map(|x| x as f64).collect(),
        SortCase::Random => {
            let mut vec = vec![];
            for _ in 0..vec_len {
                vec.push(rand::thread_rng().gen_range(0.0..100.0));
            }

            return vec;
        }
    }
}

fn show_res(sort_method: &str, len: usize, duration: Duration) {
    println!(
        "\t{} sorted array with {} elements in {:?}",
        sort_method, len, duration
    );
}
