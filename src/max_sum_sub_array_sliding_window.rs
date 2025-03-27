fn max_sum_sub_array_sliding_window(arr: &Vec<i32>, k: usize) -> Option<i32>{
    if arr.len() < k || k == 0 {
        return None
    }
    let initial_window = &arr[0..k];
    let window_sum: i32 = initial_window.iter().sum();
    let mut max_sum = window_sum;
    for i in 1..arr.len() {
        let left  = i;
        let right = i + k;
        if right > arr.len() {
            break;
        }
        let current_window_sum = arr[left..right].iter().sum();
        if current_window_sum > max_sum {
            max_sum = current_window_sum
        }
    }
    Some(max_sum)
}

pub(crate) fn run_max_sum_sub_array_sliding_window() {
    let test_cases = vec![
        (vec![2, 1, 5, 1, 3, 2], 3, Some(9)),
        (vec![1, 2, 3, 4, 5], 2, Some(9)),
        (vec![5, 5, 5, 5], 2, Some(10)),
        (vec![1, 2], 3, None), // window too big
        (vec![], 2, None),
    ];

    for (i, (arr, k, expected)) in test_cases.into_iter().enumerate() {
        let result = max_sum_sub_array_sliding_window(&arr, k);
        println!(
            "Test Case {}: {} — max sum of window size {} in {:?}",
            i + 1,
            if result == expected { "✅ Passed" } else { "❌ Failed" },
            k,
            arr
        );
    }
}