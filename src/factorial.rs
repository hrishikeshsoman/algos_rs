fn factorial(n: i32) -> i32 {
    if n < 0 {
        panic!("Factorial is not defined for negative numbers, you dumb fuck!")
    }
    else if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub(crate) fn run_factorial_tests() {
    let test_cases = vec![
        (0, Some(1)),
        (1, Some(1)),
        (2, Some(2)),
        (3, Some(6)),
        (5, Some(120)),
        (7, Some(5040)),
        (12, Some(479001600)), // Still safe for i32
        (13, None), // Overflow! i32 can't handle 13!
        (-1, None), // Negative input: should panic or be handled
    ];

    println!("---------FACTORIAL_TESTS_BEGIN---------");
    for (index, (input, expected)) in test_cases.into_iter().enumerate() {
        let result = std::panic::catch_unwind(|| factorial(input));
        let passed: bool;

        if result.is_ok() && expected.is_some() {
            let actual = result.unwrap();        // Safe because we checked is_ok()
            let expected_val = expected.unwrap(); // Safe because we checked is_some()
            passed = actual == expected_val;
        } else if result.is_err() && expected.is_none() {
            // We expected a panic, and it panicked — good!
            passed = true;
        } else {
            // Either it panicked but shouldn't have,
            // or it succeeded but returned the wrong value
            passed = false;
        }

        println!(
            "Test Case {}: {} — factorial({})",
            index + 1,
            if passed { "✅ Passed" } else { "❌ Failed" },
            input
        );
    }
    println!("---------FACTORIAL_TESTS_END---------");
}
