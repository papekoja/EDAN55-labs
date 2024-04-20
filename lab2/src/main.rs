use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{BinaryHeap, HashMap};
use std::env;
mod heap_element;
use heap_element::HeapElement;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let file_name  = &args[1];
    let path = "./data/".to_string() + file_name;
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut adjacency_map: HashMap<usize, Vec<u32>> = HashMap::new();

    for(index, line) in reader.lines().enumerate().skip(1) {
        let edges = line?.trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        adjacency_map.insert(index, edges);
    }

    // for (key, values) in &adjacency_map {
    //     println!("Key: {}", key);
    //     println!("Values: {:?}", values);
    // }

    let res = R0(adjacency_map.clone());
    println!("{}", res);
    Ok(())
}

fn R0(mut adjacency_map: HashMap<usize, Vec<u32>>) -> i32 {
    let mut heap = map_to_heap(adjacency_map);
    if let Some(next_node) = heap.pop() {
        
    } else {
        return 0;
    }
    unimplemented!("This function is not implemented yet.")
}

fn map_to_heap(mut adjacency_map: HashMap<usize, Vec<u32>>) -> BinaryHeap<HeapElement> {
    let mut heap: BinaryHeap<HeapElement> = BinaryHeap::new();
    for (key, value) in &adjacency_map {
        let count_of_ones = value.iter().filter(|&&x| x == 1).count();
        heap.push(HeapElement{index: *key, value: count_of_ones});
    }
    heap
}

fn remove_node(adjacency_map: &HashMap<usize, Vec<u32>>, key: usize, remove_neighboors: bool) -> HashMap<usize, Vec<u32>>{
    
    unimplemented!("This function is not implemented yet.")
}