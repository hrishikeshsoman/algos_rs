use std::collections::HashSet;

fn find_all_unique_substring_sliding_window(input_str: String) -> String {
    let chars: Vec<char> = input_str.chars().collect();
    for i in 0..chars.len() {
        for j in i..chars.len() {
            println!("all substrings: {:?}", &chars[i..=j])
        }
    }
    let mut window_left = 0;
    let mut window_right = 0;
    let mut seen = HashSet::new();

    for i in 0..chars.len() {
        // if !seen.contains(&chars[i]) {
        //     seen.insert(chars[i]);
        // }
        let current_window = [window_left..=window_right];










        // for j in i..chars.len() {
        //     if !seen.contains(&chars[j]) {
        //         seen.insert(chars[j]);
        //         println!("substrings with non repeating characters: {:?}", &chars[i..=j])
        //     } else {
        //         println!("Already seen: {:?}", chars[j])
        //     }
        // }
    }

    "".to_string()
}

pub(crate) fn run_find_all_unique_substring_sliding_window() {
    println!("unique substrings {}", find_all_unique_substring_sliding_window("AABC".to_string()))
}
