use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::models::arena_tree::ArenaTree;

/* Parses the .td file into nodes and edges. The nodes are put in the tree and the edges are put in a hashmap.
Then builds the edges in the tree recursively with some node as the root, right now it's set to node 1.*/
pub fn parse_td(filename: &str) -> ArenaTree<Vec<usize>> {
    let mut path: String = "./data/".to_string();
    path.push_str(&filename);

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut tree: ArenaTree<Vec<usize>> = ArenaTree::default();
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts.get(0) {
            Some(&"s") | Some(&"c") => continue,
            Some(&"b") => {
                let idx = parts[1].parse::<usize>().unwrap();
                let val = parts[2..]
                    .iter()
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to parse indices: {}", e);
                        vec![]
                    });
                tree.insert_node(idx, val);
            }
            Some(first) => {
                if !first.is_empty() {
                    let node1 = match parts[0].parse::<usize>() {
                        Ok(num) => num,
                        Err(e) => {
                            panic!("Error parsing '{}': {}", parts[0], e);
                        }
                    };
                    let node2 = match parts[1].parse::<usize>() {
                        Ok(num) => num,
                        Err(e) => {
                            panic!("Error parsing '{}': {}", parts[0], e);
                        }
                    };
                    edges.entry(node1).or_insert_with(|| vec![]).push(node2);
                    edges.entry(node2).or_insert_with(|| vec![]).push(node1);
                }
            }
            _ => eprintln!("empty line encountered!"),
        }
    }

    rec_build_edges(&mut tree, &edges, 1);
    tree
}

fn rec_build_edges(
    tree: &mut ArenaTree<Vec<usize>>,
    edges: &HashMap<usize, Vec<usize>>,
    node_idx: usize,
) {
    /* If it's not the root node and it only has one child, it's a leaf. */
    if tree.arena.get(&node_idx).unwrap().parent.is_some() && edges[&node_idx].len() == 1 {
        tree.leafs.push(node_idx);
        return;
    }

    /* take a node idx and set its edges as children as long as they're not a parent.
    set the parent to the children to this node. */
    for &child_idx in edges[&node_idx][0..].iter() {
        let node = tree.arena.get_mut(&node_idx).unwrap();
        let parent = node.parent;

        if parent != Some(child_idx) {
            node.children.push(child_idx);
            tree.arena.get_mut(&child_idx).unwrap().parent = Some(node_idx);
            rec_build_edges(tree, edges, child_idx);
        }
    }
}
