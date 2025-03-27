use std::collections::{HashMap, HashSet, VecDeque};

fn get_adjacency_list() -> HashMap<String, Vec<String>> {
    let mut adjacency_list = HashMap::new();
    adjacency_list.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
    adjacency_list.insert("B".to_string(), vec!["D".to_string(), "E".to_string()]);
    adjacency_list.insert("C".to_string(), vec![]);
    adjacency_list.insert("D".to_string(), vec![]);
    adjacency_list.insert("E".to_string(), vec!["F".to_string()]);
    adjacency_list.insert("F".to_string(), vec![]);

    adjacency_list
}

fn bfs(start: String, target: String) -> Option<String> {
    let adjacency_list = get_adjacency_list();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distance_of_node: HashMap<String, i32> = HashMap::new();
    queue.push_back(start.clone());
    distance_of_node.insert(start.clone(), 0);
    while let Some(node) = queue.pop_front() {
        let current_distance = *distance_of_node.get(&node).unwrap();
        if node == target {
            println!(
                "Found target {} at distance {} from {}",
                node,
                &distance_of_node.get(&node).unwrap().clone(),
                start
            );
            return Some(target);
        } else {
            if let Some(neighbours) = adjacency_list.get(&node) {
                for neighbour in neighbours {
                    if visited.contains(neighbour) {
                        println!("{} is already visited, skipping!", neighbour)
                    } else {
                        visited.insert(node.clone());
                        queue.push_back(neighbour.clone());
                        distance_of_node.insert(neighbour.clone(), current_distance + 1);
                    }
                }
            }
        }
    }
    println!("Target not found! :( ");
    None
}

pub fn run_bfs_tests() {
    let test_cases = vec![
        ("A", "F", Some("F")),  // Path exists: A → B → E → F
        ("A", "C", Some("C")),  // Direct neighbor
        ("A", "A", Some("A")),  // Same start and target
        ("C", "F", None),       // No path from C to F
        ("X", "A", None),       // Start node not in graph
        ("A", "X", None),       // Target node not in graph
    ];

    println!("---------BFS_BEGIN---------");

    for (i, (start, target, expected)) in test_cases.into_iter().enumerate() {
        let result = bfs(start.to_string(), target.to_string());
        println!(
            "Test Case {}: {} — Start: {}, Target: {}, Expected: {:?}, Got: {:?}",
            i + 1,
            if result == expected { "✅ Passed" } else { "❌ Failed" },
            start,
            target,
            expected,
            result,
        );
    }

    println!("---------BFS_END-----------");
}
