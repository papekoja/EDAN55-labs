use std::collections::HashMap;

use crate::models::arena_tree::{ArenaTree, Node};

// dp_table links all different independent sets to their maximum independent set weight as:
// <u128, u32> = <bitmask of independent set, max weight>
type DPTable = HashMap<u128, usize>;

pub fn algorithm(graph: &HashMap<usize, Vec<usize>>, tree: &ArenaTree<Vec<usize>>) {
    // let adj_matrix = get_adjacency_matrix(graph);
    // let bag = &tree.arena.get(&2).unwrap().val;
    // let all_subsets = all_subsets(bag);
    // println!("Subsets:");
    // for subset in &all_subsets {
    //     println!("{:?}", subset);
    // }
    // let independent_sets = get_independent_sets(&all_subsets, &adj_matrix);
    // println!("Independent sets:");
    // for independent_set in &independent_sets {
    //     println!("{:?}", independent_set);
    // }
    let adjacency_matrix = get_adjacency_matrix(graph);

    //root is always 1
    let root = tree.arena.get(&1).unwrap();
    let mut dp_tables: HashMap<usize, DPTable> = HashMap::new();
    post_order_traverse(tree, &adjacency_matrix, &mut dp_tables, root);
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
    let mut dp_table: HashMap<u128, usize> = HashMap::new();
    fill_table(tree, graph, &mut dp_table, dp_tables, node);
    dp_tables.insert(node.idx, dp_table);
}

fn fill_table(
    tree: &ArenaTree<Vec<usize>>,
    graph: &HashMap<usize, u128>,
    dp_table: &mut HashMap<u128, usize>,
    dp_tables: &HashMap<usize, DPTable>,
    node: &Node<Vec<usize>>,
) {
    let independent_sets = get_independent_sets(&node.val, graph);
    for independent_set in independent_sets {
        let set_weight = independent_set.count_ones();
        if node.children.is_empty() {
            dp_table.insert(independent_set, )
        } else {
    
        }
    }
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

fn all_subsets(bag: &Vec<usize>) -> Vec<u128> {
    let mut subsets: Vec<u128> = vec![];
    for i in 0..(1 << bag.len()) {
        let mut subset: u128 = vec![];
        for j in 0..bag.len() {
            if i & (1 << j) != 0 {
                subset = subset | (1 << j);
            }
        }
        subsets.push(subset);
    }
    subsets
}

fn bit_representation(set: &Vec<usize>) -> u128 {
    let mut bits: u128 = 0;
    for node in set {
        bits = bits | (1 << node);
    }
    bits
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

// To find a maximum weight independent set of G:
fn _f_t() {
    // given a tree decomposition (T, {Vt}) of G:
    //Modify the tree decomposition if necessary so that it is nonredundant
    //Root T at a node r
    //for each node t of T in post-order
    //  if t is a leaf then
    //      for each independent set U of Vt
    //          f_t(U)=w(U)
    //  Else
    //      for each independent set U of Vt
    //          f_t(U) is determined by the recurrence in (10.8)
    //return max{f_r(U): U subs V_r is independent}
}

/*
Gör en adjacency vektor för alla noder i en bag. (ettor och nollor)
Gör sedan alla permutationer av alla setet i bagen och solla ut alla som inte är independent set:
Det görs genom att göra AND mellan
•en vektor där alla noder som är med i setet är 1
•och adjacency vektorn för de noder som är med i setet
Ifall det finns en etta någonstans är det inte ett inependent set.

För detta har man alltså en hashmap med independent sets.
*/
