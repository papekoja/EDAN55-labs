use std::collections::HashMap;

use crate::models::arena_tree::{ArenaTree, Node};

// dp_table links all different independent sets to their maximum independent set weight as:
// <u128, u32> = <bitmask of independent set, max weight>
type DPTable = HashMap<u128, u32>;

pub fn algorithm(graph: &HashMap<usize, Vec<usize>>, tree: &ArenaTree<Vec<usize>>) {
    let adjacency_matrix = get_adjacency_matrix(graph);

    // let independent_sets = get_independent_sets(&tree.arena.get(&1).unwrap().val, &adjacency_matrix);
    // for independent_set in independent_sets {
    //     println!("Independent set: {:b}", independent_set);
    // }

    //root is always 1
    let root = tree.arena.get(&1).unwrap();
    let mut dp_tables: HashMap<usize, DPTable> = HashMap::new();
    post_order_traverse(tree, &adjacency_matrix, &mut dp_tables, root);
    // print dp_tables
    // for (node, table) in dp_tables.iter() {
    //     println!("Node: {:?}", node);
    //     for (independent_set, weight) in table.iter() {
    //         println!("Independent set: {:b}, weight: {:?}", independent_set, weight);
    //     }
    // }
    let max_weight = dp_tables.get(&1).unwrap().values().max().unwrap();
    println!("Max weight: {:?}", max_weight);
}

fn post_order_traverse(
    tree: &ArenaTree<Vec<usize>>,
    graph: &HashMap<usize, u128>,
    dp_tables: &mut HashMap<usize, DPTable>,
    node: &Node<Vec<usize>>,
) {
    for child in &node.children {
        let child_node = tree.arena.get(child).unwrap();
        post_order_traverse(tree, graph, dp_tables, child_node)
    }
    let mut dp_table: DPTable = HashMap::new();
    fill_table(tree, graph, &mut dp_table, dp_tables, node);
    dp_tables.insert(node.idx, dp_table);
}

fn fill_table(
    tree: &ArenaTree<Vec<usize>>,
    graph: &HashMap<usize, u128>,
    dp_table: &mut DPTable,
    dp_tables: &HashMap<usize, DPTable>,
    node: &Node<Vec<usize>>,
) {
    let independent_sets = get_independent_sets(&node.val, graph);

    for independent_set in independent_sets {
        let weight = independent_set.count_ones();
        if node.children.is_empty() {
            dp_table.insert(independent_set, weight);
        } else {
            let mut total_weight = weight;

            for child in &node.children {
                let mut max_weight = 0;

                let child_node = tree.arena.get(child).unwrap();
                let child_dp_table = dp_tables.get(child).unwrap();
                for (child_independent_set, child_weight) in child_dp_table.iter() {
                    if is_compatible(independent_set, *child_independent_set, child_node, tree) {
                        let weight =
                            child_weight - (child_independent_set & independent_set).count_ones();
                        if weight > max_weight {
                            max_weight = weight;
                        }
                    }
                }
                total_weight += max_weight;
            }
            dp_table.insert(independent_set, total_weight);
        }
    }
}

fn is_compatible(
    parent_independent_set: u128,
    child_independent_set: u128,
    child_node: &Node<Vec<usize>>,
    tree: &ArenaTree<Vec<usize>>,
) -> bool {
    /* From the book:
    U_i intersect V_t = U intersect V_ti */
    let child_bag = set_to_bitmask(&child_node.val);
    let parent_bag = set_to_bitmask(&tree.arena.get(&child_node.parent.unwrap()).unwrap().val);
    let intersection = parent_independent_set & child_bag;
    let child_intersection = child_independent_set & parent_bag;
    intersection == child_intersection
}

fn set_to_bitmask(set: &Vec<usize>) -> u128 {
    let mut bitmask: u128 = 0;
    for node in set {
        bitmask = bitmask | (1 << node);
    }
    bitmask
}

fn get_independent_sets(nodes: &Vec<usize>, adj_matrix: &HashMap<usize, u128>) -> Vec<u128> {
    let all_subsets = all_subsets(nodes);

    let mut independent_sets = vec![];
    for subset in &all_subsets {
        let mut is_independent = true;
        for node in nodes {
            if subset & (1 << node) != 0 {
                if subset & adj_matrix[node] != 0 {
                    is_independent = false;
                    break;
                }
            }
        }
        if is_independent {
            independent_sets.push(*subset);
        }
    }
    independent_sets
}

fn all_subsets(nodes: &Vec<usize>) -> Vec<u128> {
    let mut subsets: Vec<u128> = vec![];
    for i in 0..(1 << nodes.len()) {
        let mut subset: u128 = 0;
        for j in 0..nodes.len() {
            if i & (1 << j) != 0 {
                subset = subset | (1 << nodes[j]);
            }
        }
        subsets.push(subset);
    }
    subsets
}

fn get_adjacency_matrix(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, u128> {
    let mut adj_matrix: HashMap<usize, u128> = HashMap::new();
    for (node, neighbours) in graph.iter() {
        adj_matrix.insert(*node, 0);
        neighbours.iter().for_each(|n| {
            let adj_bits = adj_matrix.get_mut(node).unwrap();
            let neighbour_bit: u128 = 1 << n;
            *adj_bits = *adj_bits | neighbour_bit;
        });
    }
    adj_matrix
}