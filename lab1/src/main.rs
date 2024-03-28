use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
mod r;

fn main() {
    //let path = "./data/matching_1000.txt";
    let path = "./data/pw09_100.9.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let mut nodes: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if index == 0 {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let node1 = parts[0].parse::<i32>().unwrap();
        let node2 = parts[1].parse::<i32>().unwrap();
        let w = parts[2].parse::<i32>().unwrap();
        nodes.entry(node1).or_insert_with(Vec::new).push((node2, w));
        nodes.entry(node2).or_insert_with(Vec::new).push((node1, w));
    }

    /* for (key, edges) in &nodes {
        println!("Node {}: {:?}", key, edges);
    } */
    
    let result_algorithm_r: i32 = r::r(&nodes);

    println!("{}", result_algorithm_r);
}