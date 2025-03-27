fn binary_search (arr: &Vec<i32>, target: i32) -> Option<bool> {
    if arr.len() == 0 {
        return None;
    }
    let mut left  = 0;
    let mut right  = arr.len() - 1;
    while left <= right {
        let mid = left + (right- left)/2;
        let mid_val = arr[mid];
        if mid_val == target {
            return Some(true)
        } else if target < mid_val {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        } else if target > mid_val {
            left = mid + 1
        }
    }
    None
}

pub(crate) fn run_binary_search_tests() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5], 3, Some(true)),
        (vec![1, 2, 3, 4, 5], 6, None),
        (vec![], 1, None),
        (vec![1], 1, Some(true)),
        (vec![1], 0, None),
        (vec![1, 3, 5, 7, 9], 5, Some(true)),
        (vec![1, 3, 5, 7, 9], 8, None),
    ];

    println!("---------BINARY_SEARCH_BEGINS---------");
    for (i, (arr, target, expected)) in test_cases.into_iter().enumerate() {
        let result = binary_search(&arr, target);
        println!(
            "Test Case {}: {} — searching for {} in {:?}",
            i + 1,
            if result == expected { "✅ Passed" } else { "❌ Failed" },
            target,
            arr
        );
    }
    println!("---------BINARY_SEARCH_ENDS---------");
}