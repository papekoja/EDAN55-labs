use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

/* Parses the normal undirected graph from the .gr files */
pub fn parse_gr(filename: &str) -> HashMap<usize, Vec<usize>> {
    let mut path: String = "./data/".to_string();
    path.push_str(&filename);
    
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] != "c" && parts[0] != "p"  {
            let node1 = parts[0].parse::<usize>().unwrap();
            let node2 = parts[1].parse::<usize>().unwrap();
            graph.entry(node1).or_insert_with(|| vec![]).push(node2);
            graph.entry(node2).or_insert_with(|| vec![]).push(node1);
        }
    }

    graph
}