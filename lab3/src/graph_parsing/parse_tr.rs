use std::{fs::File, io::{BufRead, BufReader}};

use crate::models::arena_tree::ArenaTree;

pub fn get_tree(filename: String) {
    let mut path: String = "./data/".to_string();
    path.push_str(&filename);
    
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let mut tree = ArenaTree::<String>::default();
    
    for line in reader.lines() {
        let line = line.unwrap();
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "c" | "p" => continue,
            "b" => {
                
            },
            _ => eprint!("Parsing of line in parse_tr.rs went wrong."),
        }
        
        // if parts[0] != "c" && parts[0] != "p"  {
        //     let node1 = parts[0].parse::<usize>().unwrap();
        //     let node2 = parts[1].parse::<usize>().unwrap();
        //     graph.entry(node1).or_insert_with(|| vec![]).push(node2);
        //     graph.entry(node2).or_insert_with(|| vec![]).push(node1);
        // }
    }

    for (node, neighbours) in graph {
        println!("{} {:?}", node, neighbours);
    }
}