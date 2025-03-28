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
    None
}

pub fn run_bfs_tests() {
    println!("---------BFS_BEGIN---------");
        let result = bfs("A".to_string(), "F".to_string());
        println!(
            "Test Case 1, {}",
            if result == Some("F".to_string()) { "✅ Passed" } else { "❌ Failed" },
        );
    println!("---------BFS_END-----------");
}
