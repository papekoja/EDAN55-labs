use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_name = "g60.in";
    let path = "./data/".to_string() + file_name;
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut adjacency_map: HashMap<usize, Vec<i32>> = HashMap::new();

    for (index, line) in reader.lines().enumerate().skip(1) {
        let edges = line?
            .trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        adjacency_map.insert(index - 1, edges);
    }

    // for (key, values) in &adjacency_map {
    //     println!("Key: {}", key);
    //     println!("Values: {:?}", values);
    // }

    let res = r0(adjacency_map.clone());
    println!("{}", res);
    Ok(())
}

fn r0(mut adjacency_map: HashMap<usize, Vec<i32>>) -> i32 {
    // if the map is emtpy, return 0
    if adjacency_map.is_empty() {
        return 0;
    }

    if let Some(node) = find_degree_zero(&adjacency_map) {
        remove_node(&mut adjacency_map, node);
        return 1 + r0(adjacency_map.clone());
    }

    // find vertex with maximum degree, return the graph
    // a. without it and without its neighbours
    // b. wihtout it
    let max_node = find_degree_max(&adjacency_map);
    let mut adjacency_map_a = adjacency_map.clone();
    remove_node(&mut adjacency_map_a, max_node);

    let mut to_remove: Vec<usize> = Vec::new();
    let mut adjacency_map_b = adjacency_map_a.clone();
    if let Some(neigbours) = adjacency_map.get(&max_node) {
        neigbours.iter().enumerate().for_each(|(i, &v)| {
            if v != 0 {
                to_remove.push(i);
            }
        });
    }
    remove_nodes(&mut adjacency_map_b, to_remove);
    return r0(adjacency_map_a).max(1 + r0(adjacency_map_b));
}

fn find_degree_zero(adjacency_map: &HashMap<usize, Vec<i32>>) -> Option<usize> {
    for (node, neighbours) in adjacency_map {
        if neighbours.iter().all(|&x| x == 0) {
            return Some(*node);
        }
    }
    None
}

fn find_degree_max(adjacency_map: &HashMap<usize, Vec<i32>>) -> usize {
    let mut max_neighbours = 0;
    let mut max_node: usize = 0;
    for (node, neighbours) in adjacency_map {
        let count = neighbours.iter().filter(|&&x| x != 0).count();
        if count > max_neighbours {
            max_neighbours = count;
            max_node = *node;
        }
    }
    max_node
}

fn remove_nodes(adjacency_map: &mut HashMap<usize, Vec<i32>>, keys: Vec<usize>) {
    for &key in &keys {
        remove_node(adjacency_map, key);
    }
}

// removes a node and its neighbours
fn remove_node(adjacency_map: &mut HashMap<usize, Vec<i32>>, key: usize) {
    adjacency_map.remove(&key);
    for (_, neighbours) in adjacency_map.iter_mut() {
            neighbours[key] = 0;
    }
}