fn main() {
    let mut vec: Vec<u32> = vec![1, 2, 4, 3, 8, 6];

    //bubble_sort(&mut vec);
    //selection_sort(&mut vec);
    insertion_sort(&mut vec);
    dbg!(vec);
}

fn bubble_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) {
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

fn insertion_sort<T: std::cmp::PartialOrd + Copy + std::fmt::Display>(vec: &mut Vec<T>) {
    for i in 1..vec.len() {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}
