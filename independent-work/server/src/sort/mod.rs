mod bubble;
mod insertion;
mod merge;
mod quicksort;
mod selection;

pub use {
    bubble::bubble_sort, insertion::insertion_sort, merge::merge_sort, quicksort::quick_sort,
    selection::selection_sort,
};
