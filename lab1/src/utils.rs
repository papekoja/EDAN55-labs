use std::collections::{HashMap, HashSet};

use rand::Rng;

pub fn calculate_cut(set_a: &HashMap<i32, Vec<(i32, i32)>>, set_b: &HashMap<i32, Vec<(i32, i32)>>) -> i32 {
    let mut sum: i32 = 0;
    let mut visited_edges: HashSet<(i32, i32)> = HashSet::new();

    for (&node_id, edges) in set_a {
        for &(neighbor_id, weight) in edges {
            let rearranged_edge = if node_id < neighbor_id { (node_id, neighbor_id) } else { (neighbor_id, node_id) };
            
            if set_b.contains_key(&neighbor_id) && !visited_edges.contains(&rearranged_edge) {
                sum += weight;
                visited_edges.insert(rearranged_edge);
            }
        }
    }
    sum
}

pub fn get_random_subset(nodes: &HashMap<i32, Vec<(i32, i32)>>) -> (HashMap<i32, Vec<(i32, i32)>>, HashMap<i32, Vec<(i32, i32)>>) {
    let mut set_a: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut set_b: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for (&node_id, edges) in nodes {
        if coin_flip() {
            set_a.insert(node_id, edges.clone());
        } else {
            set_b.insert(node_id, edges.clone());
        }
    }
    (set_a, set_b)
}

pub fn coin_flip() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)
}

pub fn swap(node_id: i32 ,set_a: &mut HashMap<i32, Vec<(i32, i32)>>, set_b: &mut HashMap<i32, Vec<(i32, i32)>>) {
    if let Some(value) = set_a.remove(&node_id) {
        set_b.insert(node_id, value);
    } else if let Some(value) = set_b.remove(&node_id) {
        set_a.insert(node_id, value);
    }
}