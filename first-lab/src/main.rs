use std::time::{Duration, Instant};

use sort_methods::{bubble_sort, insertion_sort, selection_sort};

mod sort_methods;

fn main() {
    test_sort_methods();
}

fn test_sort_methods() {
    for n in [1_000, 10_000, 100_000] {
        let mut bubble_arr: Vec<u32> = (0..n).rev().collect();
        let bubble_start = Instant::now();
        bubble_sort(&mut bubble_arr);
        let bubble_end = Instant::now();
        show_res("Bubble", n, bubble_end - bubble_start);

        let mut selection_arr: Vec<u32> = (0..n).rev().collect();
        let selection_start = Instant::now();
        selection_sort(&mut selection_arr);
        let selection_end = Instant::now();
        show_res("Selection", n, selection_end - selection_start);

        let mut insertion_arr: Vec<u32> = (0..n).rev().collect();
        let insertion_start = Instant::now();
        insertion_sort(&mut insertion_arr);
        let insertion_end = Instant::now();
        show_res("Insertion", n, insertion_end - insertion_start);

        println!();
    }

    for n in [1_000, 10_000, 100_000] {
        let mut bubble_arr: Vec<f64> = (0..n).map(|x| x as f64).rev().collect();
        let bubble_start = Instant::now();
        bubble_sort(&mut bubble_arr);
        let bubble_end = Instant::now();
        show_res("Bubble float", n, bubble_end - bubble_start);

        let mut selection_arr: Vec<f64> = (0..n).map(|x| x as f64).rev().collect();
        let selection_start = Instant::now();
        selection_sort(&mut selection_arr);
        let selection_end = Instant::now();
        show_res("Selection float", n, selection_end - selection_start);

        let mut insertion_arr: Vec<f64> = (0..n).map(|x| x as f64).rev().collect();
        let insertion_start = Instant::now();
        insertion_sort(&mut insertion_arr);
        let insertion_end = Instant::now();
        show_res("Insertion float", n, insertion_end - insertion_start);

        println!();
    }
}

fn show_res(sort_method: &str, len: u32, time: Duration) {
    println!("{} sort: {} elements - Time: {:?}", sort_method, len, time);
}
