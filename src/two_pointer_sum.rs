// Always assumes the array is sorted.
// If the array is not sorted there could be a chance that we can miss valid pairs
// Sorting in the beginning can add to time complexity

fn two_pointer_sum(arr: &Vec<i32>, target: i32) -> bool {
    if arr.len() < 2 {
        return false;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return true;
        } else if sum < target {
            left += 1
        } else {
            right -= 1
        }
    }

    false
}

pub(crate) fn run_two_pointer_sum() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5], 6, true),
        (vec![1, 2, 3, 9], 8, false),
        (vec![1, 2, 4, 4], 8, true),
        (vec![], 5, false),
        (vec![5], 5, false),
        (vec![1, 2, 3, 4, 4, 5], 8, true),
        (vec![0, 1, 2, 3, 6, 10], 16, true),
        (vec![1, 2, 3, 4, 5], 10, false),
        (vec![3, 3, 3, 3], 6, true),
        (vec![1, 5, 7, -1, 5], 6, true),
    ];

    println!("---------TWO_POINTER_SUM_BEGIN---------");
    for (index, (arr, target, expected)) in test_cases.into_iter().enumerate() {
        let result = two_pointer_sum(&arr, target);
        println!(
            "Test Case {}: {} — {}",
            index + 1,
            if result == expected {
                "✅ Passed"
            } else {
                "❌ Failed"
            },
            format!("Target: {}, Array: {:?}", target, arr)
        );
    }
    println!("---------TWO_POINTER_SUM_END---------");
}
