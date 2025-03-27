mod bfs;
mod factorial;
mod binary_search;
mod two_pointer_sum;
mod hash_set_sum;
mod max_sum_sub_array_sliding_window;

fn main() {
    bfs::run_bfs_tests();
    factorial::run_factorial_tests();
    two_pointer_sum::run_two_pointer_sum();
    hash_set_sum::run_hash_set_sum();
    binary_search::run_binary_search_tests();
    max_sum_sub_array_sliding_window::run_max_sum_sub_array_sliding_window()
}
