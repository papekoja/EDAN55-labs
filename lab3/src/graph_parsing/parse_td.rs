use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::models::arena_tree::ArenaTree;

/*Will parse the .td file and read each bag from the file to a node, all the edges between the bags will be put in a hashmap.
Then some node is selected as a root node. From this root we will use the edge hashmap to set childrens and parents to each node in the tree.*/
pub fn parse_td(filename: String) {
    let mut path: String = "./data/".to_string();
    path.push_str(&filename);

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut tree: ArenaTree<Vec<usize>> = ArenaTree::default();
    let mut bag_idxs: HashMap<usize, usize> = HashMap::new();
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts.get(0) {
            Some(&"s") => continue,
            Some(&"b") => {
                let bag_id = parts[1].parse::<usize>().unwrap();
                let val = parts[2..]
                    .iter()
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to parse indices: {}", e);
                        vec![]
                    });
                let arena_idx = tree.node(val);
                bag_idxs.insert(bag_id, arena_idx);
            }
            Some(first) => {
                if !first.is_empty() {
                    let node1 = match parts[0].parse::<usize>() {
                        Ok(num) => num,
                        Err(e) => {
                            eprintln!("Error parsing '{}': {}", parts[0], e);
                            return;
                        }
                    };
                    let node2 = match parts[1].parse::<usize>() {
                        Ok(num) => num,
                        Err(e) => {
                            eprintln!("Error parsing '{}': {}", parts[0], e);
                            return;
                        }
                    };
                    edges.entry(node1).or_insert_with(|| vec![]).push(node2);
                    edges.entry(node2).or_insert_with(|| vec![]).push(node1);
                }
            }
            _ => eprintln!("empty line encountered!"),
        }
    }

    // println!("Edges: {:?}", edges);
    // println!("node idxs: {:?}", node_idxs);

    build_edges_from_root(&mut tree, &bag_idxs, &edges, 1);
    print!("Tree: ");
    tree.print_tree();
}

fn build_edges_from_root(
    tree: &mut ArenaTree<Vec<usize>>,
    bag_idxs: &HashMap<usize, usize>,
    edges: &HashMap<usize, Vec<usize>>,
    idx: usize,
) {
    let arena_root_idx = bag_idxs[&idx];
    tree.arena[arena_root_idx].children = edges[&idx].clone();
    edges[&idx].iter().for_each(|i| {
        println!("setting parent for {} to {}", i, idx);
        let child_arena_idx = bag_idxs[i];
        tree.arena[child_arena_idx].parent = Some(child_arena_idx);
        println!("parent for {} is {:?}",tree.arena[bag_idxs[i]].idx, tree.arena[bag_idxs[i]].parent);
    });
}

fn rec_build_edges(
    tree: &mut ArenaTree<Vec<usize>>,
    bag_idxs: &HashMap<usize, usize>,
    edges: &HashMap<usize, Vec<usize>>,
    idx: usize,
) {
        /* if the node has no edges except its parent, return */
        if edges[&idx].len() == 1 {
            return;
        }
    
        /* take a node idx and set its edges as children as long as they're not a parent.
        set the parent to the children to this node.*/
}
