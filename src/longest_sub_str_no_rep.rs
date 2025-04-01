// Sliding dynamic window
use std::collections::HashSet;

fn longest_sub_str_no_rep(input_str: String) -> String {
    let characters: Vec<char> = input_str.chars().collect();
    let mut window_left = 0;
    let mut window_right = 0;
    let mut seen: HashSet<char> = HashSet::new();
    for (index, c) in characters.iter().enumerate() {
        if(seen.contains(&c)){
            window_left += 1;
        } else {
            seen.insert(*c);
            window_right = index;
        }
    }

    characters[window_left..=window_right].iter().collect()

}

pub(crate) fn run_longest_sub_str_no_rep() {
    let test_cases = vec![
        ("AABCDEF", "ABCDEF"),
        ("ABBCDEF", "BCDEF"),
        ("ABABCD", "ABCD"),
        // ("AABBCDEF", "BCDEF"),
        // ("abcabcbb", "abc"),
        // ("bbbbb", "b"),
        // ("", ""),
        // ("abcdef", "abcdef"),
        // ("abba", "ab"),
        // ("dvdf", "vdf"),
    ];

    println!("----- LONGEST SUBSTRING WITHOUT REPEATS -----");
    for (i, (input, expected)) in test_cases.into_iter().enumerate() {
        let result = longest_sub_str_no_rep(input.to_string());
        println!(
            "Test Case {}: {} — Input: {}, Output: {}, Expected: {}",
            i + 1,
            if result == expected { "✅ Passed" } else { "❌ Failed" },
            input,
            result,
            expected
        );
    }
    println!("---------------------------------------------");
}
