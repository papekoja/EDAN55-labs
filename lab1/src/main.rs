use crate::utils::calculate_cut;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use utils::{get_random_subset, swap};
mod utils;

fn main() {
    // let path = "./data/matching_1000.txt";
    let path = "./data/pw09_100.9.txt"; /* Swap to instead include matching_1000 */
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let mut nodes: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    
    for (_, line) in reader.lines().enumerate().skip(1) {
        let line = line.unwrap();
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        let node1 = parts[0].parse::<i32>().unwrap();
        let node2 = parts[1].parse::<i32>().unwrap();
        let w = parts[2].parse::<i32>().unwrap();
        nodes.entry(node1).or_insert_with(Vec::new).push((node2, w));
        nodes.entry(node2).or_insert_with(Vec::new).push((node1, w));
    }
    
    println!("Algorithm  R: {}", r(&nodes));
    println!("Algorithm  S: {}", s(&nodes, false));
    println!("Algorithm RS: {}", s(&nodes, true));
}

fn r(nodes: &HashMap<i32, Vec<(i32, i32)>>) -> i32 {
    let (set_a, set_b) = get_random_subset(nodes);
    calculate_cut(&set_a, &set_b)
}

fn s(nodes: &HashMap<i32, Vec<(i32, i32)>>, do_rs: bool) -> i32 {
    let (mut set_a, mut set_b): (HashMap<i32, Vec<(i32, i32)>>, HashMap<i32, Vec<(i32, i32)>>) =
        if do_rs {
            get_random_subset(nodes)
        } else {    
            (HashMap::new(), nodes.clone())
        };
    let mut did_swap: bool = true;
    while did_swap {
        did_swap = false;

        for &node_id in nodes.keys() {
            let pre_cut = calculate_cut(&set_a, &set_b);
            swap(node_id, &mut set_a, &mut set_b);
            let post_cut = calculate_cut(&set_a, &set_b);

            if pre_cut >= post_cut {
                swap(node_id, &mut set_a, &mut set_b);
            } else {
                did_swap = true;
            }
        }
    }

    calculate_cut(&set_a, &set_b)
}
