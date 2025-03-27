use std::collections::HashSet;

fn hash_set_sum (arr: &Vec<i32>, target: i32) -> bool {
    let mut seen = HashSet::new();
    for &num in arr {
        let complement = target - num;
        if seen.contains(&complement) {
            return true;
        }
        seen.insert(num);
    }
    false
}

pub(crate) fn run_hash_set_sum() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5], 6, true),
        (vec![1, 2, 3, 9], 8, false),
        (vec![1, 2, 4, 4], 8, true),
        (vec![], 5, false),
        (vec![5], 5, false),
        (vec![3, 3, 3], 6, true),
        (vec![1, 5, 7, -1, 5], 6, true),
        (vec![10, 15, 3, 7], 17, true),
    ];

    println!("---------TARGET_SUM_HASHSET_TESTS---------");
    for (index, (arr, target, expected)) in test_cases.into_iter().enumerate() {
        let result = hash_set_sum(&arr, target);
        println!(
            "Test Case {}: {} — Target: {}, Array: {:?}",
            index + 1,
            if result == expected { "✅ Passed" } else { "❌ Failed" },
            target,
            arr
        );
    }
    println!("------------------------------------------");
}

