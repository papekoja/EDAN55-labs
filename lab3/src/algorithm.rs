use std::collections::HashMap;

// dp_table links all different independent sets to their maximum independent set weight as:
// <u128, u32> = <bitmask of independent set, max weight>
type DPTable = HashMap<u128, u32>;
type Graph = HashMap<usize, Vec<usize>>;
type ArenaTree = crate::models::arena_tree::ArenaTree<Vec<usize>>;
type Node = crate::models::arena_tree::Node<Vec<usize>>;

pub fn algorithm(graph: &HashMap<usize, Vec<usize>>, tree: &ArenaTree) {
    //root is always 1
    if let Some(root) = tree.arena.get(&1) {
        let mut dp_tables: HashMap<usize, DPTable> = HashMap::new();
        post_order_traverse(tree, graph, &mut dp_tables, root);
        let max_weight = dp_tables.get(&1).unwrap().values().max().unwrap();
        println!("α: {:?}", max_weight);
    } else {
        println!("α: 0");
    }
}

// to traverse the tree in post-order and fill the dp_table for each node
fn post_order_traverse(
    tree: &ArenaTree,
    graph: &Graph,
    dp_tables: &mut HashMap<usize, DPTable>,
    node: &Node,
) {
    for child in &node.children {
        let child_node = tree.arena.get(child).unwrap();
        post_order_traverse(tree, graph, dp_tables, child_node)
    }

    let mut dp_table: DPTable = HashMap::new();
    fill_table(tree, graph, &mut dp_table, dp_tables, node);
    dp_tables.insert(node.idx, dp_table);
}

// to fill the dp_table for each node.
// different for leafs and non-leafs
fn fill_table(
    tree: &ArenaTree,
    graph: &Graph,
    dp_table: &mut DPTable,
    dp_tables: &HashMap<usize, DPTable>,
    node: &Node,
) {
    let bag_adj_matrix = bag_adjacency_matrix(node, graph);
    let independent_sets = get_independent_sets(&bag_adj_matrix);

    for u in independent_sets {
        let weight = u.count_ones();
        if node.children.is_empty() {
            dp_table.insert(u, weight);
        } else {
            let mut total_weight = weight;
            let u_global_idxs = to_global_idxs(u, node);

            for child in &node.children {
                let mut max_weight = 0;
                let child_node = tree.arena.get(child).unwrap();
                let child_dp_table = dp_tables.get(child).unwrap();
                let u_intersect_vti = to_local_bitmask(&u_global_idxs, child_node);
                //following line means that we get a general mask of the intersection of the parent and child.
                //this we can use to quicker calculate u_i intersect v_t
                let vt_intersect_vti =
                    to_local_bitmask(&node.global_to_local.keys().copied().collect(), child_node);

                for (child_independent_set, child_weight) in child_dp_table.iter() {
                    let ui_intersect_vt = child_independent_set & vt_intersect_vti;

                    if ui_intersect_vt == u_intersect_vti {
                        let weight =
                            child_weight - (ui_intersect_vt & child_independent_set).count_ones();
                        if weight > max_weight {
                            max_weight = weight;
                        }
                    }
                }
                total_weight += max_weight;
            }
            dp_table.insert(u, total_weight);
        }
    }
}

// Global indexes are the indexes from the original graph and local indexes are the indexes in the bag
// to convert global indexes to local indexes
fn to_local_bitmask(global_idxs: &Vec<usize>, node: &Node) -> u128 {
    let mut bitmask: u128 = 0;
    for global_idx in global_idxs {
        if node.global_to_local.contains_key(global_idx) {
            let local_idx = node.global_to_local[global_idx];
            bitmask = bitmask | (1 << local_idx);
        }
    }
    bitmask
}

// to convert local indexes to global indexes
fn to_global_idxs(bitmask: u128, node: &Node) -> Vec<usize> {
    let mut global_idxs: Vec<usize> = vec![];
    for (global_idx, local_idx) in &node.global_to_local {
        //println!("bitmask: {:b}, local_idx: {:?}", bitmask, local_idx);
        if bitmask & (1 << *local_idx) != 0 {
            global_idxs.push(*global_idx);
        }
    }
    global_idxs
}

// sorts out all the independent sets from all power sets of a bag
fn get_independent_sets(bag_adj_matrix: &Vec<u128>) -> Vec<u128> {
    let all_subsets = all_subsets(bag_adj_matrix.len());
    let mut independent_sets: Vec<u128> = vec![];

    for subset in &all_subsets {
        let mut is_independent = true;
        for idx in 0..bag_adj_matrix.len() {
            if subset & (1 << idx) != 0 {
                if subset & bag_adj_matrix[idx] != 0 {
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

// generates all subsets of a set
fn all_subsets(set_size: usize) -> Vec<u128> {
    let mut subsets: Vec<u128> = vec![];
    for i in (0 as u128)..(1 << set_size) {
        let mut subset: u128 = 0;
        for j in 0..set_size {
            if i & (1 << j) != 0 {
                subset = subset | (1 << j);
            }
        }
        subsets.push(subset);
    }
    subsets
}

// generates the adjacency matrix of a bag
fn bag_adjacency_matrix(tree_node: &Node, graph: &Graph) -> Vec<u128> {
    let mut adj_matrix: Vec<u128> = vec![];
    let bag = &tree_node.val;

    for node in bag {
        let mut bitmask: u128 = 0;
        for neighbor in &graph[node] {
            if bag.contains(neighbor) {
                let local_index = tree_node.global_to_local[neighbor];
                bitmask = bitmask | (1 << local_index);
            }
        }
        adj_matrix.push(bitmask);
    }
    adj_matrix
}
